// is-extended.js - ES Module for Deno's deno_core
import extended from "./extended.js";

function defineIsa(extended) {
    const toStr = Object.prototype.toString;
    const hasOwn = Object.prototype.hasOwnProperty;
    const slice = Array.prototype.slice;

    function argsToArray(args, offset = 0) {
        return Array.prototype.slice.call(args, offset);
    }

    function keys(obj) {
        const ret = [];
        for (const i in obj) {
            if (hasOwn.call(obj, i)) {
                ret.push(i);
            }
        }
        return ret;
    }

    function isArguments(obj) {
        return toStr.call(obj) === "[object Arguments]" || (obj && hasOwn.call(obj, "callee"));
    }

    function isDate(obj) {
        return toStr.call(obj) === "[object Date]";
    }

    function isRegExp(obj) {
        return toStr.call(obj) === "[object RegExp]";
    }

    function isString(obj) {
        return toStr.call(obj) === "[object String]";
    }

    function isUndefined(obj) {
        return typeof obj === "undefined";
    }

    function isNull(obj) {
        return obj === null;
    }

    function isUndefinedOrNull(obj) {
        return isUndefined(obj) || isNull(obj);
    }

    function deepEqual(actual, expected) {
        if (actual === expected) return true;
        if (isDate(actual) && isDate(expected)) return actual.getTime() === expected.getTime();
        if (isRegExp(actual) && isRegExp(expected)) {
            return actual.source === expected.source &&
                actual.global === expected.global &&
                actual.multiline === expected.multiline &&
                actual.lastIndex === expected.lastIndex &&
                actual.ignoreCase === expected.ignoreCase;
        }
        if (isString(actual) && isString(expected)) return actual === expected;
        if (typeof actual !== "object" || typeof expected !== "object") return actual === expected;
        return objEquiv(actual, expected);
    }

    function objEquiv(a, b) {
        if (isUndefinedOrNull(a) || isUndefinedOrNull(b)) return false;
        if (a.prototype !== b.prototype) return false;
        if (isArguments(a)) {
            if (!isArguments(b)) return false;
            return deepEqual(slice.call(a), slice.call(b));
        }
        const ka = keys(a);
        const kb = keys(b);
        if (ka.length !== kb.length) return false;
        ka.sort();
        kb.sort();
        for (let i = 0; i < ka.length; i++) if (ka[i] !== kb[i]) return false;
        for (let i = 0; i < ka.length; i++) if (!deepEqual(a[ka[i]], b[ka[i]])) return false;
        return true;
    }

    const isFunction = obj => toStr.call(obj) === "[object Function]" || typeof obj === "function";
    const isObject = obj => obj !== null && typeof obj === "object";
    const isArray = Array.isArray || (obj => toStr.call(obj) === "[object Array]");
    const isBoolean = obj => obj === true || obj === false || toStr.call(obj) === "[object Boolean]";
    const isHash = obj => isObject(obj) && obj.constructor === Object;
    const isEmpty = obj => (
        isArguments(obj) ? obj.length === 0 :
        isArray(obj) || isString(obj) ? obj.length === 0 :
        isObject(obj) ? keys(obj).length === 0 :
        true
    );
    const isDefined = obj => !isUndefined(obj);
    const isInstanceOf = (obj, clazz) => isFunction(clazz) && obj instanceof clazz;
    const isNumber = obj => toStr.call(obj) === "[object Number]";
    const isTrue = obj => obj === true;
    const isFalse = obj => obj === false;
    const isNotNull = obj => obj !== null;
    const isEq = (a, b) => a == b;
    const isNeq = (a, b) => a != b;
    const isSeq = (a, b) => a === b;
    const isSneq = (a, b) => a !== b;
    const isLt = (a, b) => a < b;
    const isLte = (a, b) => a <= b;
    const isGt = (a, b) => a > b;
    const isGte = (a, b) => a >= b;
    const isIn = (item, arr) => {
        if (isString(arr) || (isArray(arr) && Array.prototype.indexOf)) return arr.indexOf(item) !== -1;
        for (let i = 0; i < arr.length; i++) if (arr[i] == item) return true;
        return false;
    };
    const isNotIn = (item, arr) => !isIn(item, arr);
    const isLike = (obj, reg) => isString(reg) ? !!obj.match(reg) : isRegExp(reg) && reg.test(obj);
    const isNotLike = (obj, reg) => !isLike(obj, reg);
    const contains = isIn;
    const notContains = isNotIn;
    const containsAt = (arr, item, i) => isArray(arr) && arr.length > i && arr[i] == item;
    const notContainsAt = (arr, item, i) => isArray(arr) && arr[i] != item;
    const has = (obj, prop) => hasOwn.call(obj, prop);
    const notHas = (obj, prop) => !has(obj, prop);
    const isLength = (obj, l) => has(obj, "length") && obj.length === l;
    const isNotLength = (obj, l) => has(obj, "length") && obj.length !== l;

    const isa = {
        isFunction, isObject, isEmpty, isHash, isNumber, isString, isDate,
        isArray, isBoolean, isUndefined, isDefined, isUndefinedOrNull,
        isNull, isArguments, instanceOf: isInstanceOf, isRegExp,
        deepEqual, isTrue, isFalse, isNotNull,
        isEq, isNeq, isSeq, isSneq, isIn, isNotIn,
        isLt, isLte, isGt, isGte, isLike, isNotLike,
        contains, notContains, has, notHas,
        isLength, isNotLength, containsAt, notContainsAt
    };

    const tester = {
        constructor() { this._testers = []; },
        noWrap: {
            tester() {
                const list = this._testers;
                return value => list.some(fn => fn(value));
            }
        }
    };

    const switcher = {
        constructor() {
            this._cases = [];
            this.__default = null;
        },
        def(fn) {
            this.__default = fn;
        },
        noWrap: {
            switcher() {
                const cases = this._cases, def = this.__default;
                return (...args) => {
                    for (const c of cases) {
                        const [shouldBreak, result] = c(args);
                        if (shouldBreak) return result;
                    }
                    return def && def(...args);
                };
            }
        }
    };

    for (const key in isa) {
        if (hasOwn.call(isa, key)) {
            tester[key] = function () { this._testers.push(isa[key]); };
            switcher[key] = function (...args) {
                let fn = args.pop(), doBreak = true;
                if (typeof fn === "boolean") {
                    doBreak = fn;
                    fn = args.pop();
                }
                if (!isFunction(fn)) throw new TypeError("handler must be function");
                this._cases.push(input => {
                    return isa[key](...input.concat(args)) ? [doBreak, fn(...input)] : [false];
                });
            };
        }
    }

    const is = extended.define(isa).expose(isa);
    is.tester = extended.define(tester);
    is.switcher = extended.define(switcher);

    return is;
}

const isExtended = defineIsa(extended);
export default isExtended;
