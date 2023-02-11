import { aFunction } from "../dir1/mod1";

export function anotherFunction(arg1: string) {
    return `I just call aFunction: ${aFunction(arg1)}`;
}
