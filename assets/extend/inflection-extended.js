// inflection-extended.js - ES Module for deno_core

import extended from "./extended.js";
import is from "./is-extended.js";
import array from "./array-extended.js";
import args from "./arguments-extended.js";

const isUndefinedOrNull = is.isUndefinedOrNull;
const CAMELIZE_CONVERT_REGEXP = /_(.)/g;
const DASH = "-";
const UNDERSCORE = "_";
const UNDERSCORE_CONVERT_REGEXP1 = /([A-Z]+)(\d+|[A-Z][a-z])/g;
const UNDERSCORE_CONVERT_REGEXP2 = /(\d+|[a-z])(\d+|[A-Z])/g;
const UNDERSCORE_CONVERT_REPLACE = "$1_$2";

let PLURALS = [];
let SINGULARS = [];
let UNCOUNTABLES = [];

function _plural(rule, replacement) {
    PLURALS.unshift([rule, replacement]);
}

function _singular(rule, replacement) {
    SINGULARS.unshift([rule, replacement]);
}

function _irregular(singular, plural) {
    _plural(new RegExp("(" + singular.charAt(0) + ")" + singular.slice(1) + "$"), "$1" + plural.slice(1));
    _singular(new RegExp("(" + plural.charAt(0) + ")" + plural.slice(1) + "$"), "$1" + singular.slice(1));
}

function _uncountable(...words) {
    UNCOUNTABLES = array.flatten([...UNCOUNTABLES, ...words]);
}

// Plural rules
_plural(/$/, 's');
_plural(/s$/i, 's');
_plural(/(alias|(?:stat|octop|vir|b)us)$/i, '$1es');
_plural(/(buffal|tomat)o$/i, '$1oes');
_plural(/([ti])um$/i, '$1a');
_plural(/sis$/i, 'ses');
_plural(/(?:([^f])fe|([lr])f)$/i, '$1$2ves');
_plural(/(hive)$/i, '$1s');
_plural(/([^aeiouy]|qu)y$/i, '$1ies');
_plural(/(x|ch|ss|sh)$/i, '$1es');
_plural(/(matr|vert|ind)ix|ex$/i, '$1ices');
_plural(/([m|l])ouse$/i, '$1ice');
_plural(/^(ox)$/i, "$1en");

// Singular rules
_singular(/s$/i, '');
_singular(/([ti])a$/i, '$1um');
_singular(/(analy|ba|cri|diagno|parenthe|progno|synop|the)ses$/i, '$1sis');
_singular(/([^f])ves$/i, '$1fe');
_singular(/([h|t]ive)s$/i, '$1');
_singular(/([lr])ves$/i, '$1f');
_singular(/([^aeiouy]|qu)ies$/i, '$1y');
_singular(/(m)ovies$/i, '$1ovie');
_singular(/(x|ch|ss|sh)es$/i, '$1');
_singular(/([m|l])ice$/i, '$1ouse');
_singular(/buses$/i, 'bus');
_singular(/oes$/i, 'o');
_singular(/shoes$/i, 'shoe');
_singular(/(alias|(?:stat|octop|vir|b)us)es$/i, '$1');
_singular(/(vert|ind)ices$/i, '$1ex');
_singular(/matrices$/i, 'matrix');

// Irregulars
_irregular('person', 'people');
_irregular('man', 'men');
_irregular('child', 'children');
_irregular('sex', 'sexes');
_irregular('move', 'moves');
_irregular('quiz', 'quizzes');
_irregular('testis', 'testes');

// Uncountables
_uncountable("equipment", "information", "rice", "money", "species", "series", "fish", "sheep", "news");

function camelize(str) {
    return isUndefinedOrNull(str) ? str : str.replace(CAMELIZE_CONVERT_REGEXP, (_, b) => b.toUpperCase());
}

function underscore(str) {
    return isUndefinedOrNull(str)
        ? str
        : str
            .replace(UNDERSCORE_CONVERT_REGEXP1, UNDERSCORE_CONVERT_REPLACE)
            .replace(UNDERSCORE_CONVERT_REGEXP2, UNDERSCORE_CONVERT_REPLACE)
            .replace(DASH, UNDERSCORE)
            .toLowerCase();
}

function classify(str) {
    return isUndefinedOrNull(str)
        ? str
        : camelize(singularize(str.replace(/.*\./g, '')));
}

function pluralize(str) {
    if (isUndefinedOrNull(str)) return str;
    if (array.indexOf(UNCOUNTABLES, str) !== -1) return str;

    for (const [rule, replacement] of PLURALS) {
        const newStr = str.replace(rule, replacement);
        if (newStr !== str) return newStr;
    }
    return str;
}

function singularize(str) {
    if (isUndefinedOrNull(str)) return str;
    if (array.indexOf(UNCOUNTABLES, str) !== -1) return str;

    for (const [rule, replacement] of SINGULARS) {
        const newStr = str.replace(rule, replacement);
        if (newStr !== str) return newStr;
    }
    return str;
}

const inflect = {
    singular: _singular,
    plural: _plural,
    uncountable: _uncountable,
    camelize,
    underscore,
    classify,
    pluralize,
    singularize
};

const inflectionExtended = extended.define(is.isString, inflect).expose(inflect);
export default inflectionExtended;
