// Since version 2.12 they switched to using 'preserveModules = true'
// https://github.com/AzureAD/microsoft-authentication-library-for-js/pull/3563
// This means the the old index.es.js file doesn't exist and instead modules imports are left
// file://./node_modules/@azure/msal-browser/dist/index.js
// so need to rebundle into a single ES Module file that can be used by bindgen

import { nodeResolve } from "@rollup/plugin-node-resolve";

export default [
    {
        input: "./node_modules/@azure/msal-browser/dist/index.js",
        output: {
            file: "js/msal-browser.js",
            format: "es",
            banner: "'use strict';",
            sourcemap: true,
        },
        plugins: [
            nodeResolve()
        ]
    }
];
