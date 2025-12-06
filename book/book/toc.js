// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/structure.html"><strong aria-hidden="true">3.</strong> Project Structure</a></li><li class="chapter-item expanded affix "><li class="part-title">Core Concepts</li><li class="chapter-item expanded "><a href="concepts/prs-format.html"><strong aria-hidden="true">4.</strong> The PRS Format</a></li><li class="chapter-item expanded "><a href="concepts/scene-structure.html"><strong aria-hidden="true">5.</strong> Scene Structure</a></li><li class="chapter-item expanded "><a href="concepts/iiur-principles.html"><strong aria-hidden="true">6.</strong> IIUR Principles</a></li><li class="chapter-item expanded "><a href="concepts/recipe-context.html"><strong aria-hidden="true">7.</strong> Recipe Context</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Category A: Scene Creation</li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/index.html"><strong aria-hidden="true">8.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/minimal-scene.html"><strong aria-hidden="true">9.</strong> Minimal Scene</a></li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/grid-layout.html"><strong aria-hidden="true">10.</strong> Grid Layout</a></li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/flex-layout.html"><strong aria-hidden="true">11.</strong> Flex Layout</a></li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/absolute-layout.html"><strong aria-hidden="true">12.</strong> Absolute Layout</a></li><li class="chapter-item expanded "><a href="recipes/a-scene-creation/nested-layouts.html"><strong aria-hidden="true">13.</strong> Nested Layouts</a></li><li class="chapter-item expanded affix "><li class="part-title">Category B: Widget Configuration</li><li class="chapter-item expanded "><a href="recipes/b-widget-config/index.html"><strong aria-hidden="true">14.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/b-widget-config/text-input.html"><strong aria-hidden="true">15.</strong> Text Input</a></li><li class="chapter-item expanded "><a href="recipes/b-widget-config/slider.html"><strong aria-hidden="true">16.</strong> Slider</a></li><li class="chapter-item expanded "><a href="recipes/b-widget-config/dropdown.html"><strong aria-hidden="true">17.</strong> Dropdown</a></li><li class="chapter-item expanded "><a href="recipes/b-widget-config/charts.html"><strong aria-hidden="true">18.</strong> Charts</a></li><li class="chapter-item expanded "><a href="recipes/b-widget-config/data-table.html"><strong aria-hidden="true">19.</strong> Data Table</a></li><li class="chapter-item expanded affix "><li class="part-title">Category C: Resources</li><li class="chapter-item expanded "><a href="recipes/c-resources/index.html"><strong aria-hidden="true">20.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/c-resources/local-model.html"><strong aria-hidden="true">21.</strong> Local Model</a></li><li class="chapter-item expanded "><a href="recipes/c-resources/remote-model.html"><strong aria-hidden="true">22.</strong> Remote Model</a></li><li class="chapter-item expanded "><a href="recipes/c-resources/dataset.html"><strong aria-hidden="true">23.</strong> Dataset</a></li><li class="chapter-item expanded "><a href="recipes/c-resources/fallback-sources.html"><strong aria-hidden="true">24.</strong> Fallback Sources</a></li><li class="chapter-item expanded affix "><li class="part-title">Category D: Bindings</li><li class="chapter-item expanded "><a href="recipes/d-bindings/index.html"><strong aria-hidden="true">25.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/d-bindings/simple-update.html"><strong aria-hidden="true">26.</strong> Simple Update</a></li><li class="chapter-item expanded "><a href="recipes/d-bindings/debounced.html"><strong aria-hidden="true">27.</strong> Debounced</a></li><li class="chapter-item expanded "><a href="recipes/d-bindings/model-inference.html"><strong aria-hidden="true">28.</strong> Model Inference</a></li><li class="chapter-item expanded "><a href="recipes/d-bindings/chain-actions.html"><strong aria-hidden="true">29.</strong> Chain Actions</a></li><li class="chapter-item expanded "><a href="recipes/d-bindings/conditional.html"><strong aria-hidden="true">30.</strong> Conditional</a></li><li class="chapter-item expanded affix "><li class="part-title">Category E: Theming</li><li class="chapter-item expanded "><a href="recipes/e-theming/index.html"><strong aria-hidden="true">31.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/e-theming/dark-preset.html"><strong aria-hidden="true">32.</strong> Dark Preset</a></li><li class="chapter-item expanded "><a href="recipes/e-theming/light-preset.html"><strong aria-hidden="true">33.</strong> Light Preset</a></li><li class="chapter-item expanded "><a href="recipes/e-theming/custom-colors.html"><strong aria-hidden="true">34.</strong> Custom Colors</a></li><li class="chapter-item expanded "><a href="recipes/e-theming/custom-fonts.html"><strong aria-hidden="true">35.</strong> Custom Fonts</a></li><li class="chapter-item expanded affix "><li class="part-title">Category F: Permissions</li><li class="chapter-item expanded "><a href="recipes/f-permissions/index.html"><strong aria-hidden="true">36.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/f-permissions/network.html"><strong aria-hidden="true">37.</strong> Network</a></li><li class="chapter-item expanded "><a href="recipes/f-permissions/filesystem.html"><strong aria-hidden="true">38.</strong> Filesystem</a></li><li class="chapter-item expanded "><a href="recipes/f-permissions/minimal.html"><strong aria-hidden="true">39.</strong> Minimal</a></li><li class="chapter-item expanded affix "><li class="part-title">Category G: Expressions</li><li class="chapter-item expanded "><a href="recipes/g-expressions/index.html"><strong aria-hidden="true">40.</strong> Overview</a></li><li class="chapter-item expanded "><a href="recipes/g-expressions/select.html"><strong aria-hidden="true">41.</strong> Select</a></li><li class="chapter-item expanded "><a href="recipes/g-expressions/filter.html"><strong aria-hidden="true">42.</strong> Filter</a></li><li class="chapter-item expanded "><a href="recipes/g-expressions/sort-limit.html"><strong aria-hidden="true">43.</strong> Sort &amp; Limit</a></li><li class="chapter-item expanded "><a href="recipes/g-expressions/aggregation.html"><strong aria-hidden="true">44.</strong> Aggregation</a></li><li class="chapter-item expanded "><a href="recipes/g-expressions/format.html"><strong aria-hidden="true">45.</strong> Format</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/api.html"><strong aria-hidden="true">46.</strong> API Documentation</a></li><li class="chapter-item expanded "><a href="reference/errors.html"><strong aria-hidden="true">47.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="reference/features.html"><strong aria-hidden="true">48.</strong> Feature Flags</a></li><li class="chapter-item expanded affix "><li class="part-title">Appendix</li><li class="chapter-item expanded "><a href="appendix/toyota-way.html"><strong aria-hidden="true">49.</strong> Toyota Way Principles</a></li><li class="chapter-item expanded "><a href="appendix/qa-checklist.html"><strong aria-hidden="true">50.</strong> Recipe QA Checklist</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
