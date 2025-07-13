function createDeclared() {
    const arraySlice = Array.prototype.slice;
    let classCounter = 0;
    const SUPER_REGEXP = /(super)/g;

    function isArray(obj) {
        return Object.prototype.toString.call(obj) === "[object Array]";
    }

    function isObject(obj) {
        return obj !== null && typeof obj === "object";
    }

    function isHash(obj) {
        return isObject(obj) && obj.constructor === Object;
    }

    function indexOf(arr, item) {
        for (let i = 0, l = arr.length; i < l; i++) {
            if (arr[i] === item) return i;
        }
        return -1;
    }

    function callSuper(args) {
        const meta = this.__meta;
        if (!meta || !meta.supers) return;
        const supers = meta.supers;
        let pos = meta.superMeta.pos;
        const name = meta.superMeta.name;
        const f = meta.superMeta.f;
        while (pos < supers.length) {
            let m = supers[pos][name];
            if (typeof m === "function" && (m = m._f || m) !== f) {
            meta.superMeta.pos = pos + 1;
            return m.apply(this, args);
            }
            pos++;
        }
        return null;
    }

    function getSuper() {
        const meta = this.__meta;
        const supers = meta.supers;
        let pos = meta.superMeta.pos;
        const name = meta.superMeta.name;
        const f = meta.superMeta.f;
        while (pos < supers.length) {
            let m = supers[pos][name];
            if (typeof m === "function" && (m = m._f || m) !== f) {
                meta.superMeta.pos = pos + 1;
                return m.bind(this);
            }
            pos++;
        }
        return null;
    }

    function getter(name) {
        const g = this.__getters__;
        return g.hasOwnProperty(name) ? g[name].call(this) : this[name];
    }

    function setter(name, val) {
        const s = this.__setters__;
        if (isHash(name)) {
            for (const k in name) {
                if (s.hasOwnProperty(k)) s[k].call(this, name[k]);
                else this[k] = name[k];
            }
        } else {
            if (s.hasOwnProperty(name)) return s[name].call(this, val);
            else return (this[name] = val);
        }
    }

    function defaultFunction() {
        return callSuper.call(this, arguments);
    }

    function functionWrapper(f, name) {
        if (!SUPER_REGEXP.test(f.toString())) {
            f._f = f;
            return f;
        }
        const wrapper = function () {
            const meta = this.__meta;
            const orig = meta.superMeta;
            meta.superMeta = { f, pos: 0, name };
            const ret = f.apply(this, arguments);
            meta.superMeta = orig;
            return ret;
        };
        wrapper._f = f;
        return wrapper;
    }

    function defineProps(child, proto) {
        const s = proto.setters || {},
            g = proto.getters || {};
        for (const k in s) child.__setters__[k] = s[k];
        for (const k in g) child.__getters__[k] = g[k];
        for (const i in proto) {
            if (i !== "getters" && i !== "setters") {
                const fn = proto[i];
                child[i] = typeof fn === "function" ? functionWrapper(fn, i) : fn;
            }
        }
    }

    function getNew(ctor) {
        function Temp() {}
        Temp.prototype = ctor.prototype;
        return new Temp();
    }

    function __declare(child, sup, proto, getBase) {
        const unique = `declare${++classCounter}`;
        const bases = [],
            staticBases = [];
        const instanceSupers = [],
            staticSupers = [];
        const meta = {
            supers: instanceSupers,
            unique,
            bases,
            superMeta: { f: null, pos: 0, name: null },
        };
        const childMeta = {
            supers: staticSupers,
            unique,
            bases: staticBases,
            isConstructor: true,
            superMeta: { f: null, pos: 0, name: null },
        };

        let childProto = {};
        if (isHash(sup) && !proto) {
            proto = sup;
            sup = getBase(); // 延迟获取 Base
        }
        if (typeof sup === "function" || isArray(sup)) {
            const supers = isArray(sup) ? sup : [sup];
            sup = supers.shift();
            childProto = getNew(sup);
            child.__meta = childMeta;
            childProto.__meta = meta;
            childProto.__getters__ = {};
            childProto.__setters__ = {};
            child.__getters__ = {};
            child.__setters__ = {};
        }

        child.prototype = childProto;
        if (proto) {
            const inst = proto.instance || {};
            const stat = proto.static || {};
            defineProps(childProto, inst);
            defineProps(child, stat);
            childProto.constructor = functionWrapper(
                inst.constructor || defaultFunction,
                "constructor"
            );
            child.init = functionWrapper(stat.init || defaultFunction, "init");
        } else {
            child.init = functionWrapper(defaultFunction, "init");
            childProto.constructor = functionWrapper(defaultFunction, "constructor");
        }

        childProto._super = child._super = callSuper;
        childProto._getSuper = child._getSuper = getSuper;
        childProto._static = child;
    }

    let Base = null;

    function declare(sup, proto) {
        function declared() {
            this.constructor.apply(this, arguments);
        }
        __declare(declared, sup, proto, () => Base);
        return declared.init() || declared;
    }

    declare.singleton = function singleton(sup, proto) {
        let instance;
        function Singleton() {
            if (!instance) {
                this.constructor.apply(this, arguments);
                instance = this;
            }
            return instance;
        }
        __declare(Singleton, sup, proto, () => Base);
        return Singleton.init() || Singleton;
    };

    Base = declare({
        instance: { get: getter, set: setter },
        static: {
            get: getter,
            set: setter,
            mixin: function () {
                return this;
            },
            extend: function (proto) {
                return declare(this, proto);
            },
            as: function (obj, name) {
                if (obj && name) obj[name] = this;
                else obj.exports = this;
                return this;
            },
        },
    });

    return declare;
}

export default createDeclared();
