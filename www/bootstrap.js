// we will load this to index.js so we can catch some error
// now we need to change the entry point of the applicaton

import("./index").catch((e) => console.error("Error importing index.js :", e));
