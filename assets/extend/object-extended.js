// object-extended.js - ES Module for deno_core

import extended from "./extended.js";
import is from "./is-extended.js";
import array from "./array-extended.js";

const { deepEqual, isString, isHash, isFunction, isObject } = is;
const { difference } = array;
const hasOwn = Object.prototype.hasOwnProperty;

function _merge(target, source) {
    for (const name in source) {
        if (hasOwn.call(source, name)) {
            const s = source[name];
            if (!(name in target) || target[name] !== s) {
                target[name] = s;
            }
        }
    }
    return target;
}

function _deepMerge(target, source) {
    for (const name in source) {
        if (hasOwn.call(source, name)) {
            const s = source[name];
            const t = target[name];
            if (!deepEqual(t, s)) {
                if (isHash(t) && isHash(s)) {
                    target[name] = _deepMerge(t, s);
                } else if (isHash(s)) {
                    target[name] = _deepMerge({}, s);
                } else {
                    target[name] = s;
                }
            }
        }
    }
    return target;
}

function merge(obj, ...args) {
    if (!obj) obj = {};
    for (const arg of args) _merge(obj, arg);
    return obj;
}

function deepMerge(obj, ...args) {
    if (!obj) obj = {};
    for (const arg of args) _deepMerge(obj, arg);
    return obj;
}

function extend(parent, child) {
    const proto = parent.prototype || parent;
    merge(proto, child);
    return parent;
}

function keys(hash) {
    if (!isHash(hash)) throw new TypeError();
    const ret = [];
    for (const i in hash) {
        if (hasOwn.call(hash, i)) {
            ret.push(i);
        }
    }
    return ret;
}

function forEach(hash, iterator, scope) {
    if (!isHash(hash) || !isFunction(iterator)) throw new TypeError();
    const objKeys = keys(hash);
    for (const key of objKeys) {
        iterator.call(scope || hash, hash[key], key, hash);
    }
    return hash;
}

function filter(hash, iterator, scope) {
    if (!isHash(hash) || !isFunction(iterator)) throw new TypeError();
    const objKeys = keys(hash);
    const ret = {};
    for (const key of objKeys) {
        const value = hash[key];
        if (iterator.call(scope || hash, value, key, hash)) {
            ret[key] = value;
        }
    }
    return ret;
}

function values(hash) {
    if (!isHash(hash)) throw new TypeError();
    const objKeys = keys(hash);
    return objKeys.map(k => hash[k]);
}

function invert(hash) {
    if (!isHash(hash)) throw new TypeError();
    const objKeys = keys(hash);
    const ret = {};
    for (const key of objKeys) {
        ret[hash[key]] = key;
    }
    return ret;
}

function toArray(hash) {
    if (!isHash(hash)) throw new TypeError();
    const objKeys = keys(hash);
    return objKeys.map(key => [key, hash[key]]);
}

function omit(hash, omitted) {
    if (!isHash(hash)) throw new TypeError();
    if (isString(omitted)) omitted = [omitted];
    const objKeys = difference(keys(hash), omitted);
    const ret = {};
    for (const key of objKeys) {
        ret[key] = hash[key];
    }
    return ret;
}

const hash = {
    forEach,
    filter,
    invert,
    values,
    toArray,
    keys,
    omit
};

const obj = {
    extend,
    merge,
    deepMerge,
    omit
};

const ret = extended
    .define(isObject, obj)
    .define(isHash, hash)
    .define(isFunction, { extend })
    .expose({ hash })
    .expose(obj);

const origExtend = ret.extend;
ret.extend = function (...args) {
    if (args.length === 1) {
        return origExtend.apply(ret, args);
    } else {
        return extend(...args);
    }
};

export default ret;
