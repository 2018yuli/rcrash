// extended-base.js - ES Module for Deno Core

import declare from "./declare.js";

const slice = Array.prototype.slice;
const undef = undefined;

function indexOf(arr, item) {
    for (let i = 0, l = arr.length; i < l; i++) {
        if (arr[i] === item) return i;
    }
    return -1;
}

function isArray(obj) {
    return Object.prototype.toString.call(obj) === "[object Array]";
}

const merge = (function () {
    function _merge(target, source, exclude) {
        for (const name in source) {
            if (Object.prototype.hasOwnProperty.call(source, name) && indexOf(exclude, name) === -1) {
                const s = source[name];
                if (!(name in target) || target[name] !== s) {
                    target[name] = s;
                }
            }
        }
        return target;
    }

    return function merge(obj, ...sources) {
        if (!obj) obj = {};
        let exclude = sources[sources.length - 1];
        if (isArray(exclude)) {
            sources.pop();
        } else {
            exclude = [];
        }
        for (const src of sources) {
            _merge(obj, src, exclude);
        }
        return obj;
    };
})();

function defineExtender() {
    function extender(supers = []) {
        const Base = declare({
            instance: {
                constructor(value) {
                    this._value = value;
                },
                value() {
                    return this._value;
                },
                eq(val) {
                    return this.__extender__(this._value === val);
                },
                neq(val) {
                    return this.__extender__(this._value !== val);
                },
                print() {
                    console.log(this._value);
                    return this;
                }
            }
        });

        let defined = [];

        function addMethod(proto, name, fn) {
            if (typeof fn !== "function") throw new TypeError("Must provide a function");
            proto[name] = name === "constructor"
                ? function () {
                    this._super(arguments);
                    fn.apply(this, arguments);
                }
                : function (...args) {
                    args.unshift(this._value);
                    const ret = fn.apply(this, args);
                    return ret !== undef ? this.__extender__(ret) : this;
                };
        }

        function addNoWrapMethod(proto, name, fn) {
            if (typeof fn !== "function") throw new TypeError("Must provide a function");
            proto[name] = name === "constructor"
                ? function () {
                    this._super(arguments);
                    fn.apply(this, arguments);
                }
                : function (...args) {
                    args.unshift(this._value);
                    return fn.apply(this, args);
                };
        }

        function decorateProto(proto, decoration, nowrap = false) {
            for (const i in decoration) {
                if (Object.prototype.hasOwnProperty.call(decoration, i)) {
                    if (i === "noWrap") {
                        decorateProto(proto, decoration[i], true);
                    } else if (i === "getters" || i === "setters") {
                        proto[i] = decoration[i];
                    } else if (nowrap) {
                        addNoWrapMethod(proto, i, decoration[i]);
                    } else {
                        addMethod(proto, i, decoration[i]);
                    }
                }
            }
        }

        function _extender(obj) {
            if (obj instanceof Base) return obj;
            let OurBase = Base;
            for (const [tester, proto] of defined) {
                if (tester(obj)) {
                    OurBase = OurBase.extend({ instance: proto });
                }
            }
            const instance = new OurBase(obj);
            instance.__extender__ = _extender;
            return instance;
        }

        function always() { return true; }

        function define(tester, decorate) {
            if (typeof tester === "object") {
                decorate = tester;
                tester = always;
            }
            decorate = decorate || {};
            const proto = {};
            decorateProto(proto, decorate);
            if (!proto.hasOwnProperty("constructor") && decorate.hasOwnProperty("constructor")) {
                addMethod(proto, "constructor", decorate.constructor);
            }
            defined.push([tester, proto]);
            return _extender;
        }

        function extend(supr) {
            if (supr && supr.__defined__) {
                defined = defined.concat(supr.__defined__);
            }
            merge(_extender, supr, ["define", "extend", "expose", "__defined__"]);
            return _extender;
        }

        _extender.define = define;
        _extender.extend = extend;
        _extender.expose = function (...methodsList) {
            for (const methods of methodsList) {
                if (typeof methods === "object") {
                    merge(_extender, methods, ["define", "extend", "expose", "__defined__"]);
                }
            }
            return _extender;
        };
        _extender.__defined__ = defined;

        return _extender;
    }

    return {
        define(...args) {
            return extender().define(...args);
        },
        extend(supr) {
            return extender().define().extend(supr);
        }
    };
}

export default defineExtender();
