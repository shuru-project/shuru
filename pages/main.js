// Mobile menu toggle
const menuToggle = document.querySelector('.menu-toggle');
const sidebar = document.querySelector('.sidebar');
const content = document.querySelector('.content');

menuToggle.addEventListener('click', () => {
    sidebar.classList.toggle('active');
});

// Close sidebar when clicking outside
document.addEventListener('click', (e) => {
    if (sidebar.classList.contains('active') &&
        !sidebar.contains(e.target) &&
        !menuToggle.contains(e.target)) {
        sidebar.classList.remove('active');
    }
});

// Close sidebar when clicking a link (mobile)
const sidebarLinks = sidebar.querySelectorAll('a');
sidebarLinks.forEach(link => {
    link.addEventListener('click', () => {
        if (window.innerWidth <= 768) {
            sidebar.classList.remove('active');
        }
    });
});

// Custom element for code blocks
class CodeBlock extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        const style = document.createElement('style');
        style.textContent = `
            pre {
                background-color: var(--code-block-bg-color);
                color: var(--code-block-text-color);
                padding: 20px;
                border-radius: 6px;
                overflow-x: auto;
                white-space: pre-wrap;
                word-wrap: break-word;
                font-size: 14px;
                position: relative;
            }

            .copy-btn {
                position: absolute;
                top: 10px;
                right: 10px;
                background-color: var(--button-bg-color);
                color: white;
                border: none;
                padding: 5px 10px;
                cursor: pointer;
                border-radius: 4px;
                font-size: 12px;
            }

            .copy-btn:hover {
                background-color: var(--button-hover-color);
            }

            @media (max-width: 768px) {
                pre {
                    padding: 15px;
                    font-size: 13px;
                }

                .copy-btn {
                    padding: 4px 8px;
                    font-size: 11px;
                }
            }
        `;
        this.shadowRoot.appendChild(style);

        const wrapper = document.createElement('div');
        wrapper.innerHTML = `
            <pre><code><slot></slot></code></pre>
        `;
        this.shadowRoot.appendChild(wrapper);

        const preElements = this.shadowRoot.querySelectorAll('pre');
        preElements.forEach((preElement) => {
            const copyBtn = document.createElement('button');
            copyBtn.textContent = 'Copy';
            copyBtn.classList.add('copy-btn');
            copyBtn.addEventListener('click', () => {
                this.copyToClipboard(this.textContent, copyBtn);
            });
            preElement.appendChild(copyBtn);
        });
    }

    async copyToClipboard(text, button) {
        try {
            await navigator.clipboard.writeText(text);
            button.textContent = 'Copied!';

            setTimeout(() => {
                button.textContent = 'Copy';
            }, 1000);
        } catch (err) {
            console.error('Failed to copy: ', err);
            button.textContent = 'Error';

            setTimeout(() => {
                button.textContent = 'Copy';
            }, 1000);
        }
    }
}

customElements.define('code-block', CodeBlock);
