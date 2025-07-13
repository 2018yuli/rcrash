// string-extended.js - ES Module for deno_core

import extended from "./extended.js";
import is from "./is-extended.js";
import date from "./date-extended.js";
import arr from "./array-extended.js";

const defineString = (extended, is, date, arr) => {
    const stringify = JSON.stringify;
    const isHash = is.isHash;
    const aSlice = Array.prototype.slice;

    const FORMAT_REGEX = /%((?:-?\+?.?\d*)?|(?:\[[^\[|\]]*\]))?([sjdDZ])/g;
    const INTERP_REGEX = /\{(?:\[([^\[|\]]*)\])?(\w+)\}/g;
    const STR_FORMAT = /(-?)(\+?)([A-Z|a-z|\W]?)([1-9][0-9]*)?$/;
    const OBJECT_FORMAT = /([1-9][0-9]*)$/g;

    function pad(string, length, ch, end) {
        string = "" + string;
        ch = ch || " ";
        while (string.length < length) {
            string = end ? string + ch : ch + string;
        }
        return string;
    }

    function truncate(string, length, end) {
        string = "" + string;
        if (string.length > length) {
            return end ? string.slice(-length) : string.slice(0, length);
        }
        return string;
    }

    function formatString(string, format) {
        let ret = string;
        if (STR_FORMAT.test(format)) {
            const [, isLeft, , padChar, width] = format.match(STR_FORMAT);
            const len = parseInt(width, 10);
            if (ret.length < len) ret = pad(ret, len, padChar, isLeft);
            else ret = truncate(ret, len);
        }
        return ret;
    }

    function formatNumber(number, format) {
        if (!is.isNumber(number)) throw new Error("Expected number");
        let ret = number.toString();
        const [, isLeft, signed, padChar, width] = format.match(STR_FORMAT) || [];
        if (signed) ret = (number > 0 ? "+" : "") + ret;
        const len = parseInt(width, 10);
        if (ret.length < len) ret = pad(ret, len, padChar || "0", isLeft);
        else ret = truncate(ret, len);
        return ret;
    }

    function formatObject(obj, format) {
        const spacing = parseInt((format.match(OBJECT_FORMAT) || [])[0]) || 0;
        return stringify(obj, null, spacing);
    }

    function format(str, obj) {
        if (Array.isArray(obj)) {
            let i = 0;
            return str.replace(FORMAT_REGEX, (m, format, type) => {
                const replacer = obj[i++] ?? m;
                switch (type) {
                    case "s": return formatString(replacer, format);
                    case "d": return formatNumber(replacer, format);
                    case "j": return formatObject(replacer, format);
                    case "D": return date.format(replacer, format);
                    case "Z": return date.format(replacer, format, true);
                    default: return replacer + "";
                }
            });
        } else if (isHash(obj)) {
            return str.replace(INTERP_REGEX, (m, format, key) => {
                const value = obj[key];
                if (value === undefined) return m;
                if (format) {
                    if (is.isString(value)) return formatString(value, format);
                    if (is.isNumber(value)) return formatNumber(value, format);
                    if (is.isDate(value)) return date.format(value, format);
                    if (is.isObject(value)) return formatObject(value, format);
                }
                return "" + value;
            });
        } else {
            return format(str, aSlice.call(arguments, 1));
        }
    }

    function toArray(str, delim) {
        return str && str.indexOf(delim) > -1
            ? str.replace(/\s+/g, "").split(delim)
            : [str];
    }

    function multiply(str, times) {
        return new Array(times + 1).join(str);
    }

    function style(str, opts) {
        const styles = {
            bold: 1, italic: 3, underline: 4, red: 31, green: 32, blue: 34
        };
        if (!opts) return str;
        if (Array.isArray(str)) return str.map(s => style(s, opts));
        if (Array.isArray(opts)) return opts.reduce((s, o) => style(s, o), str);
        if (styles[opts]) return `\x1B[${styles[opts]}m${str}\x1B[0m`;
        return str;
    }

    function escape(str, except) {
        return str.replace(/([.$?*|{}()\[\]\\/+^])/g, ch =>
            except && arr.indexOf(except, ch) !== -1 ? ch : "\\" + ch);
    }

    const string = {
        toArray,
        pad,
        truncate,
        multiply,
        format,
        style,
        escape,
        trim: str => str.trim(),
        trimLeft: str => str.trimStart(),
        trimRight: str => str.trimEnd(),
        isEmpty: str => str.length === 0
    };

    return extended.define(is.isString, string).define(is.isArray, { style }).expose(string).expose({ characters: {} });
};

export default defineString(extended, is, date, arr);
