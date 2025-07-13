// array-extended.mjs

import extended from './extended.js';
import is from './is-extended.js';
import args from './arguments-extended.js';

export default (function () {
    "use strict";

    const isString = is.isString;
    const isArray = Array.isArray || is.isArray;
    const isDate = is.isDate;
    const floor = Math.floor;
    const abs = Math.abs;
    const mathMax = Math.max;
    const mathMin = Math.min;

    const arrayProto = Array.prototype;
    const arrayIndexOf = arrayProto.indexOf;
    const arrayForEach = arrayProto.forEach;
    const arrayMap = arrayProto.map;
    const arrayReduce = arrayProto.reduce;
    const arrayReduceRight = arrayProto.reduceRight;
    const arrayFilter = arrayProto.filter;
    const arrayEvery = arrayProto.every;
    const arraySome = arrayProto.some;

    const argsToArray = args.argsToArray;

    function cross(num, cros) {
        return reduceRight(cros, function (a, b) {
            if (!isArray(b)) b = [b];
            b.unshift(num);
            a.unshift(b);
            return a;
        }, []);
    }

    function permute(num, cross, length) {
        return cross.map((_, i) => [num].concat(rotate(cross, i)).slice(0, length));
    }

    function intersection(a, b) {
        return a.filter(aOne => indexOf(b, aOne) !== -1);
    }

    function _sort(arr, property) {
        const isAll = (arr, test) => every(arr, test);
        const defaultCmp = (a, b) => a - b;
        const dateSort = (a, b) => a.getTime() - b.getTime();

        if (!isArray(arr)) return [];

        const ret = arr.slice();
        if (property) {
            if (typeof property === "function") {
                return ret.sort(property);
            }
            return ret.sort((a, b) => {
                const aProp = a[property], bProp = b[property];
                if (isString(aProp) && isString(bProp)) return aProp > bProp ? 1 : aProp < bProp ? -1 : 0;
                if (isDate(aProp) && isDate(bProp)) return aProp.getTime() - bProp.getTime();
                return aProp - bProp;
            });
        }

        if (isAll(ret, isString)) return ret.sort();
        if (isAll(ret, isDate)) return ret.sort(dateSort);
        return ret.sort(defaultCmp);
    }

    function indexOf(arr, searchElement, from) {
        let index = (from || 0) - 1;
        while (++index < arr.length) {
            if (arr[index] === searchElement) return index;
        }
        return -1;
    }

    function lastIndexOf(arr, searchElement, from) {
        const t = Object(arr);
        const len = t.length >>> 0;
        if (len === 0) return -1;

        let n = len;
        if (arguments.length > 2) {
            n = Number(arguments[2]);
            if (isNaN(n)) n = 0;
            else n = (n > 0 || -1) * floor(abs(n));
        }

        let k = n >= 0 ? mathMin(n, len - 1) : len - abs(n);

        while (k >= 0) {
            if (k in t && t[k] === searchElement) return k;
            k--;
        }
        return -1;
    }

    function filter(arr, iterator, scope) {
        if (!isArray(arr) || typeof iterator !== "function") throw new TypeError();
        return arrayFilter && arrayFilter === arr.filter
            ? arr.filter(iterator, scope)
            : arr.reduce((res, val, i) => {
                    if (iterator.call(scope, val, i, arr)) res.push(val);
                    return res;
                }, []);
    }

    function forEach(arr, iterator, scope) {
        if (!isArray(arr) || typeof iterator !== "function") throw new TypeError();
        if (arrayForEach && arrayForEach === arr.forEach) return arr.forEach(iterator, scope), arr;
        for (let i = 0, len = arr.length; i < len; ++i) {
            iterator.call(scope || arr, arr[i], i, arr);
        }
        return arr;
    }

    function every(arr, iterator, scope) {
        if (!isArray(arr) || typeof iterator !== "function") throw new TypeError();
        return arrayEvery && arrayEvery === arr.every
            ? arr.every(iterator, scope)
            : !arr.some((v, i) => !iterator.call(scope, v, i, arr));
    }

    function some(arr, iterator, scope) {
        if (!isArray(arr) || typeof iterator !== "function") throw new TypeError();
        return arraySome && arraySome === arr.some
            ? arr.some(iterator, scope)
            : arr.some((v, i) => iterator.call(scope, v, i, arr));
    }

    function map(arr, iterator, scope) {
        if (!isArray(arr) || typeof iterator !== "function") throw new TypeError();
        return arrayMap && arrayMap === arr.map
            ? arr.map(iterator, scope)
            : arr.reduce((res, val, i) => {
                    res.push(iterator.call(scope, val, i, arr));
                    return res;
                }, []);
    }

    function reduce(arr, accumulator, curr) {
        if (!isArray(arr) || typeof accumulator !== "function") throw new TypeError();
        return arrayReduce && arrayReduce === arr.reduce
            ? arguments.length > 2 ? arr.reduce(accumulator, curr) : arr.reduce(accumulator)
            : arr.slice(1).reduce(accumulator, arguments.length < 3 ? arr[0] : curr);
    }

    function reduceRight(arr, accumulator, curr) {
        if (!isArray(arr) || typeof accumulator !== "function") throw new TypeError();
        return arrayReduceRight && arrayReduceRight === arr.reduceRight
            ? arguments.length > 2 ? arr.reduceRight(accumulator, curr) : arr.reduceRight(accumulator)
            : arr.slice(0, -1).reduceRight(accumulator, arguments.length < 3 ? arr[arr.length - 1] : curr);
    }

    function toArray(o) {
        const args = argsToArray(arguments);
        if (args.length === 1) {
            if (isArray(o)) return o;
            if (is.isHash(o)) return Object.entries(o);
            return [o];
        }
        return args.flatMap(toArray);
    }

    function sum(arr = []) {
        return arr.length ? reduce(arr, (a, b) => a + b) : 0;
    }

    function avg(arr = []) {
        if (!arr.length) return 0;
        const total = sum(arr);
        if (!is.isNumber(total)) throw new Error("Cannot average an array of non numbers.");
        return total / arr.length;
    }

    function sort(arr, cmp) {
        return _sort(arr, cmp);
    }

    function min(arr, cmp) {
        return _sort(arr, cmp)[0];
    }

    function max(arr, cmp) {
        return _sort(arr, cmp)[arr.length - 1];
    }

    function difference(arr1, ...rest) {
        const args = flatten(rest);
        return isArray(arr1) ? filter(arr1, a => indexOf(args, a) === -1) : arr1;
    }

    function removeDuplicates(arr) {
        return arr.filter((item, index) => indexOf(arr, item) === index);
    }

    const unique = removeDuplicates;

    function rotate(arr, n = 1) {
        const ret = arr.slice();
        while (n > 0) ret.push(ret.shift()), n--;
        while (n < 0) ret.unshift(ret.pop()), n++;
        return ret;
    }

    function permutations(arr, length = arr.length) {
        const copy = arr.slice();
        return arr.reduce((a, b, i) => {
            const perm = length > 1
                ? permute(b, rotate(copy, i).slice(1), length)
                : [[b]];
            return a.concat(perm);
        }, []);
    }

    function zip(...args) {
        const [arr1, ...rest] = args;
        return arr1.map((v, i) => [v, ...rest.map(a => a?.[i] ?? null)]);
    }

    function transpose(arr) {
        const ret = [];
        if (!arr?.length) return ret;
        arr.forEach(row => {
            row?.forEach((val, i) => {
                if (!ret[i]) ret[i] = [];
                ret[i].push(val);
            });
        });
        return ret;
    }

    function valuesAt(arr, ...indexes) {
        return indexes.map(i => arr[i] ?? null);
    }

    function union(...args) {
        return removeDuplicates(args.flat());
    }

    function intersect(...args) {
        return args.reduce((a, b) => intersection(a, b));
    }

    function powerSet(arr) {
        return arr.reduce((a, b) => a.concat(a.map(x => [...x, b])), [[]]);
    }

    function cartesian(a, b) {
        return a.length && b.length
            ? cross(a[0], b).concat(cartesian(a.slice(1), b))
            : [];
    }

    function compact(arr) {
        return arr.filter(x => !is.isUndefinedOrNull(x));
    }

    function multiply(arr, times = 1) {
        return Array.from({ length: times }, () => arr).flat();
    }

    function flatten(...args) {
        return args.flat();
    }

    function pluck(arr, path) {
        const props = path.split(".");
        return props.reduce((res, p) => res.map(x => typeof x?.[p] === 'function' ? x[p]() : x[p]), arr);
    }

    function invoke(arr, func, ...a) {
        return arr.map(item => (typeof func === "string" ? item[func] : func).apply(item, a));
    }

    const array = {
        toArray, sum, avg, sort, min, max, difference, removeDuplicates, unique,
        rotate, permutations, zip, transpose, valuesAt, union, intersect,
        powerSet, cartesian, compact, multiply, flatten, pluck, invoke,
        forEach, map, filter, reduce, reduceRight, some, every, indexOf, lastIndexOf
    };

    return extended.define(isArray, array).expose(array);
})();
