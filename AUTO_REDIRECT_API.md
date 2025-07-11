# Auto Redirect API

This is the Javascript/CSS/HTML API to use to automatically redirect when one land on GHWSEE that scripters, extension authors, and power users may want to use.

Run this on the domain https://github-wiki-see.page and it will automatically redirect to GitHub.com's wiki page when landing from a search page onto an indexable GHWSEE page and won't fire on other pages such as the front page.

```javascript
if (document.getElementById('header_button'))
    window.location.replace(document.querySelector(".visit_url_original").href);
```

This API will be maintained as the mirror page changes or gets updated. The ID names and class names used in the example above will stay this and will not change for the foreseeable future.

## Examples

Here are some examples of this "API" in use.

Please contribute if you have other examples of using this API with other setups and ecosystems.

### Page Extender.app

https://github.com/fphilipe/PageExtender.app

PageExtender is a Safari Extension that injects CSS and JS files into websites, allowing you to customize your favorite websites to your needs.

Create a file: `github-wiki-see.page.js`

The file is named so that it only runs on the domain `github-wiki-see.page` and not on any other domain.

Contents: Use the example Javascript at the top of this document.

See [@gingerbeardman](https://github.com/gingerbeardman)'s post for the original post but note it uses an older version of the example Javascript which may have back button issues:

https://github.com/nelsonjchen/github-wiki-see-rs/issues/136#issuecomment-1040821971

### Userscript
```javascript
// ==UserScript==
// @name         github-wiki-see-redirect
// @namespace    nelsonjchen.github-wiki-see-redirect
// @version      0.2
// @description
// @author       nelsonjchen, firepup650
// @match        https://github-wiki-see.page/*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=github.com
// @grant        none
// ==/UserScript==
(function() {
    'use strict';
    if (document.getElementById('header_button')) {
        window.location.replace(document.querySelector(".visit_url_original").href);
    }
})();
```
