use actix_web::{get, HttpResponse, Responder};
use crate::config::constant::{EXAMPLE_DOIS, SERVICE_NAME, SERVICE_DESCRIPTION, SERVICE_TITLE};

#[get("/")]
pub async fn index() -> impl Responder {
    let examples_html = EXAMPLE_DOIS
        .iter()
        .map(|doi| format!(r#"<span class="example-item" onclick="fillDOI('{}')">{}</span>"#, doi, doi))
        .collect::<Vec<_>>()
        .join("\n            ");

    let html = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{SERVICE_NAME}} - {{SERVICE_TITLE}}</title>
    <meta name="description" content="{{SERVICE_DESCRIPTION}} Download research papers instantly using DOI.">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        :root {
            --primary: #667eea;
            --secondary: #764ba2;
            --accent: #f093fb;
            --dark: #1a1a2e;
            --light: #ffffff;
            --gray-50: #fafbfc;
            --gray-100: #f8f9fa;
            --gray-200: #e9ecef;
            --gray-300: #dee2e6;
            --gray-400: #ced4da;
            --gray-500: #adb5bd;
            --gray-600: #6c757d;
            --gray-700: #495057;
            --gray-800: #343a40;
            --gray-900: #212529;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: #0f0f1e;
            color: var(--gray-900);
            overflow-x: hidden;
            line-height: 1.6;
        }

        /* Animated Background */
        .background {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: -1;
            background: linear-gradient(125deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
            animation: gradientShift 20s ease infinite;
            background-size: 300% 300%;
        }

        @keyframes gradientShift {
            0% { background-position: 0% 50%; }
            50% { background-position: 100% 50%; }
            100% { background-position: 0% 50%; }
        }

        .background::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48cGF0dGVybiBpZD0iZ3JpZCIgd2lkdGg9IjEwMCIgaGVpZ2h0PSIxMDAiIHBhdHRlcm5Vbml0cz0idXNlclNwYWNlT25Vc2UiPjxwYXRoIGQ9Ik0gMTAwIDAgTCAwIDAgMCAxMDAiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2ZmZmZmZjEwIiBzdHJva2Utd2lkdGg9IjEiLz48L3BhdHRlcm4+PC9kZWZzPjxyZWN0IHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiIGZpbGw9InVybCgjZ3JpZCkiLz48L3N2Zz4=');
            opacity: 0.1;
        }

        /* Floating particles */
        .particles {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            z-index: -1;
        }

        .particle {
            position: absolute;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 50%;
            animation: float 20s infinite;
        }

        @keyframes float {
            0%, 100% {
                transform: translateY(0) translateX(0) scale(1);
                opacity: 0;
            }
            10% {
                opacity: 1;
            }
            90% {
                opacity: 1;
            }
            100% {
                transform: translateY(-100vh) translateX(100px) scale(0.3);
            }
        }

        /* Navigation */
        nav {
            position: fixed;
            top: 0;
            width: 100%;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            box-shadow: 0 1px 40px rgba(0, 0, 0, 0.05);
            z-index: 1000;
            padding: 1rem 2rem;
            transition: all 0.3s ease;
        }

        nav.scrolled {
            background: rgba(255, 255, 255, 0.98);
            box-shadow: 0 5px 40px rgba(0, 0, 0, 0.1);
        }

        .nav-container {
            max-width: 1400px;
            margin: 0 auto;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .logo {
            font-size: 1.5rem;
            font-weight: 800;
            background: linear-gradient(135deg, var(--primary), var(--secondary));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .nav-links {
            display: flex;
            gap: 2rem;
            list-style: none;
        }

        .nav-links a {
            color: var(--gray-700);
            text-decoration: none;
            font-weight: 500;
            transition: color 0.3s;
        }

        .nav-links a:hover {
            color: var(--primary);
        }

        /* Main Container */
        .main-container {
            padding-top: 80px;
            min-height: 100vh;
        }

        /* Hero Section */
        .hero {
            padding: 100px 20px;
            text-align: center;
            position: relative;
        }

        .hero-content {
            max-width: 900px;
            margin: 0 auto;
            animation: fadeInUp 1s ease;
        }

        @keyframes fadeInUp {
            from {
                opacity: 0;
                transform: translateY(30px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        h1 {
            font-size: clamp(2.5rem, 6vw, 4.5rem);
            font-weight: 900;
            margin-bottom: 1.5rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            line-height: 1.2;
            animation: textGlow 3s ease infinite alternate;
        }

        @keyframes textGlow {
            from { filter: brightness(1); }
            to { filter: brightness(1.2); }
        }

        .hero-subtitle {
            font-size: clamp(1.1rem, 2vw, 1.4rem);
            color: rgba(255, 255, 255, 0.95);
            margin-bottom: 3rem;
            font-weight: 400;
            letter-spacing: 0.5px;
        }

        /* Search Card */
        .search-card {
            background: rgba(255, 255, 255, 0.98);
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            border-radius: 24px;
            padding: 2.5rem;
            max-width: 700px;
            margin: 0 auto 4rem;
            box-shadow:
                0 30px 60px rgba(0, 0, 0, 0.12),
                0 10px 20px rgba(0, 0, 0, 0.08);
            border: 1px solid rgba(255, 255, 255, 0.5);
            animation: fadeInUp 1.2s ease;
        }

        .input-wrapper {
            position: relative;
            margin-bottom: 1rem;
        }

        .input-icon {
            position: absolute;
            left: 1.5rem;
            top: 50%;
            transform: translateY(-50%);
            color: var(--primary);
            font-size: 1.5rem;
            z-index: 1;
        }

        input[type="text"] {
            width: 100%;
            padding: 1.75rem 1.75rem 1.75rem 4rem;
            border: 3px solid transparent;
            border-radius: 20px;
            font-size: 1.25rem;
            transition: all 0.3s;
            background: linear-gradient(white, white) padding-box,
                        linear-gradient(135deg, var(--primary), var(--secondary)) border-box;
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.1);
            font-weight: 500;
        }

        input[type="text"]:focus {
            outline: none;
            box-shadow: 0 0 0 4px rgba(102, 126, 234, 0.15),
                        0 15px 40px rgba(102, 126, 234, 0.2);
            transform: translateY(-2px);
        }

        input[type="text"]::placeholder {
            color: var(--gray-400);
            font-weight: 400;
        }

        .button-group {
            display: flex;
            gap: 1rem;
        }

        button {
            flex: 1;
            padding: 1.25rem 2.5rem;
            border: none;
            border-radius: 14px;
            font-size: 1.15rem;
            font-weight: 700;
            cursor: pointer;
            transition: all 0.3s;
            position: relative;
            overflow: hidden;
            letter-spacing: 0.5px;
        }

        button::before {
            content: '';
            position: absolute;
            top: 50%;
            left: 50%;
            width: 0;
            height: 0;
            border-radius: 50%;
            background: rgba(255, 255, 255, 0.3);
            transform: translate(-50%, -50%);
            transition: width 0.6s, height 0.6s;
        }

        button:active::before {
            width: 300px;
            height: 300px;
        }

        .btn-primary {
            background: linear-gradient(135deg, var(--primary), var(--secondary));
            color: white;
            box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
        }

        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(102, 126, 234, 0.4);
        }

        .btn-secondary {
            background: var(--gray-100);
            color: var(--gray-700);
            border: 2px solid var(--gray-300);
        }

        .btn-secondary:hover {
            background: var(--gray-200);
            border-color: var(--gray-400);
        }

        /* Status Messages */
        .status {
            margin-top: 1.5rem;
            padding: 1rem 1.5rem;
            border-radius: 12px;
            display: none;
            animation: slideIn 0.4s ease;
            font-weight: 500;
        }

        @keyframes slideIn {
            from {
                opacity: 0;
                transform: translateY(-10px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .status.success {
            background: linear-gradient(135deg, #00b894, #00cec9);
            color: white;
        }

        .status.error {
            background: linear-gradient(135deg, #ff6b6b, #ee5a24);
            color: white;
        }

        .status.info {
            background: linear-gradient(135deg, #74b9ff, #0984e3);
            color: white;
        }

        /* Examples Section - Inline Version */
        .examples-inline {
            margin: 0.75rem 0 1.25rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }

        .examples-label {
            color: var(--gray-500);
            font-size: 0.7rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            white-space: nowrap;
        }

        .examples-grid {
            display: flex;
            flex-wrap: wrap;
            gap: 0.3rem;
            align-items: center;
        }

        .example-item {
            display: inline-block;
            padding: 0.2rem 0.5rem;
            background: var(--gray-100);
            border-radius: 4px;
            font-size: 0.7rem;
            cursor: pointer;
            transition: all 0.2s;
            border: 1px solid var(--gray-200);
            font-weight: 500;
            color: var(--gray-600);
            font-family: 'Courier New', monospace;
            white-space: nowrap;
            line-height: 1.2;
        }

        .example-item:hover {
            background: var(--primary);
            color: white;
            border-color: var(--primary);
            transform: translateY(-1px);
            box-shadow: 0 2px 6px rgba(102, 126, 234, 0.15);
        }

        /* API Section */
        .api-section {
            padding: 4rem 2rem;
            max-width: 1000px;
            margin: 0 auto;
        }

        .api-card {
            background: rgba(255, 255, 255, 0.98);
            backdrop-filter: blur(20px);
            border-radius: 20px;
            padding: 3rem;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
        }

        .api-card h3 {
            font-size: 1.8rem;
            margin-bottom: 2rem;
            color: var(--gray-900);
        }

        .code-block {
            background: #1e1e2e;
            color: #a6e3a1;
            padding: 1.5rem;
            border-radius: 12px;
            font-family: 'Courier New', monospace;
            margin-bottom: 1.5rem;
            position: relative;
            overflow-x: auto;
        }

        .code-block::before {
            content: 'ENDPOINT';
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            font-size: 0.7rem;
            color: rgba(255, 255, 255, 0.4);
            letter-spacing: 1px;
        }

        /* Footer */
        footer {
            background: rgba(0, 0, 0, 0.3);
            backdrop-filter: blur(10px);
            color: rgba(255, 255, 255, 0.8);
            padding: 3rem 2rem;
            text-align: center;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
        }

        .footer-content {
            max-width: 1200px;
            margin: 0 auto;
        }

        .footer-links {
            display: flex;
            justify-content: center;
            gap: 3rem;
            margin-bottom: 2rem;
            flex-wrap: wrap;
        }

        .footer-links a {
            color: rgba(255, 255, 255, 0.8);
            text-decoration: none;
            transition: color 0.3s;
        }

        .footer-links a:hover {
            color: white;
        }

        /* Responsive Design */
        @media (max-width: 768px) {
            nav {
                padding: 1rem;
            }

            .nav-links {
                display: none;
            }

            .hero {
                padding: 60px 20px;
            }

            .search-card {
                padding: 2rem 1.5rem;
            }

            .features-grid {
                grid-template-columns: 1fr;
            }

            .stats-grid {
                grid-template-columns: repeat(2, 1fr);
            }

            .button-group {
                flex-direction: column;
            }

            button {
                width: 100%;
            }
        }

        /* Loading Animation */
        .loader {
            width: 48px;
            height: 48px;
            border: 3px solid rgba(255, 255, 255, 0.3);
            border-top-color: white;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin: 2rem auto;
            display: none;
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        /* Scroll Indicator */
        .scroll-indicator {
            position: absolute;
            bottom: 2rem;
            left: 50%;
            transform: translateX(-50%);
            animation: bounce 2s infinite;
        }

        @keyframes bounce {
            0%, 20%, 50%, 80%, 100% { transform: translateX(-50%) translateY(0); }
            40% { transform: translateX(-50%) translateY(-10px); }
            60% { transform: translateX(-50%) translateY(-5px); }
        }

        .scroll-indicator svg {
            width: 30px;
            height: 30px;
            fill: rgba(255, 255, 255, 0.5);
        }
    </style>
</head>
<body>
    <!-- Background Elements -->
    <div class="background"></div>
    <div class="particles" id="particles"></div>

    <!-- Navigation -->
    <nav id="navbar">
        <div class="nav-container">
            <div class="logo">{{SERVICE_NAME}}</div>
            <ul class="nav-links">
                <li><a href="#home" onclick="return handleNavClick(event, 'home')">Home</a></li>
                <li><a href="#api" onclick="return handleNavClick(event, 'api')">API</a></li>
            </ul>
        </div>
    </nav>

    <!-- Main Container -->
    <div class="main-container">
        <!-- Hero Section -->
        <section class="hero" id="home">
            <div class="hero-content">
                <h1>{{SERVICE_TITLE}}</h1>
                <p class="hero-subtitle">{{SERVICE_DESCRIPTION}}</p>

                <!-- Search Card -->
                <div class="search-card">
                    <form id="doiForm">
                        <div class="input-wrapper">
                            <span class="input-icon">📄</span>
                            <input
                                type="text"
                                id="doiInput"
                                placeholder="Enter DOI (e.g., 10.1038/nature12373)"
                                autocomplete="off"
                            />
                        </div>

                        <!-- Examples moved inside form -->
                        <div class="examples-inline">
                            <span class="examples-label">Examples:</span>
                            <div class="examples-grid">
                                {{EXAMPLE_DOIS}}
                            </div>
                        </div>

                        <div class="button-group">
                            <button type="submit" class="btn-primary">
                                <span>Download</span>
                            </button>
                            <button type="button" class="btn-secondary" onclick="clearForm()">
                                Clear
                            </button>
                        </div>
                    </form>

                    <div id="status" class="status"></div>
                </div>
            </div>

            <div class="scroll-indicator">
                <svg viewBox="0 0 24 24">
                    <path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"/>
                </svg>
            </div>
        </section>

        <!-- API Section -->
        <section class="api-section" id="api">
            <div class="api-card">
                <h3>🚀 API Documentation</h3>
                <p style="color: var(--gray-600); margin-bottom: 2rem;">Integrate our service into your workflow with our simple REST API.</p>

                <div class="code-block">
                    <code>GET /v1/doi/{doi}</code>
                </div>

                <p style="margin-bottom: 1rem;"><strong>Example Request:</strong></p>
                <div class="code-block">
                    <code>curl -O https://yourdomain.com/v1/doi/10.1038/nature12373</code>
                </div>

                <p style="margin-bottom: 1rem;"><strong>Response:</strong></p>
                <p style="color: var(--gray-600);">Returns the PDF file directly with appropriate headers for download.</p>
            </div>
        </section>
    </div>

    <!-- Footer -->
    <footer>
        <div class="footer-content">
            <div class="footer-links">
                <a href="#home" onclick="return handleNavClick(event, 'home')">Home</a>
                <a href="#api" onclick="return handleNavClick(event, 'api')">API</a>
                <a href="https://github.com" target="_blank">GitHub</a>
                <a href="mailto:contact@example.com">Contact</a>
            </div>
            <p id="copyright">© 2024 {{SERVICE_NAME}}. Built with passion for open science.</p>
        </div>
    </footer>

    <script>
        // Update copyright year
        document.getElementById('copyright').textContent = `© ${new Date().getFullYear()} {{SERVICE_NAME}}. Built with passion for open science.`;

        // Particle Animation
        const particlesContainer = document.getElementById('particles');
        const particleCount = 20;

        for (let i = 0; i < particleCount; i++) {
            const particle = document.createElement('div');
            particle.className = 'particle';
            particle.style.width = Math.random() * 10 + 5 + 'px';
            particle.style.height = particle.style.width;
            particle.style.left = Math.random() * 100 + '%';
            particle.style.animationDelay = Math.random() * 20 + 's';
            particle.style.animationDuration = Math.random() * 20 + 20 + 's';
            particlesContainer.appendChild(particle);
        }

        // Navbar scroll effect
        window.addEventListener('scroll', () => {
            const navbar = document.getElementById('navbar');
            if (window.scrollY > 50) {
                navbar.classList.add('scrolled');
            } else {
                navbar.classList.remove('scrolled');
            }
        });

        // Handle navigation clicks with smooth scrolling
        function handleNavClick(event, targetId) {
            event.preventDefault();
            const target = document.getElementById(targetId);
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
            return false;
        }

        // Form handling
        const doiForm = document.getElementById('doiForm');
        const doiInput = document.getElementById('doiInput');
        const statusDiv = document.getElementById('status');

        doiForm.addEventListener('submit', async (e) => {
            e.preventDefault();

            const doi = doiInput.value.trim();

            if (!doi) {
                showStatus('Please enter a DOI', 'error');
                return;
            }

            if (!doi.startsWith('10.') || !doi.includes('/')) {
                showStatus('Invalid DOI format. Must start with "10." and contain "/"', 'error');
                return;
            }

            showStatus('Fetching paper...', 'info');

            try {
                const url = `/v1/doi/${doi}`;
                const response = await fetch(url);

                if (response.ok) {
                    const blob = await response.blob();
                    const downloadUrl = window.URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = downloadUrl;
                    a.download = `${doi.replace(/\//g, '_')}.pdf`;
                    document.body.appendChild(a);
                    a.click();
                    window.URL.revokeObjectURL(downloadUrl);
                    document.body.removeChild(a);
                    showStatus('PDF downloaded successfully!', 'success');
                } else {
                    const errorText = await response.text();
                    showStatus(`Error: ${errorText || response.statusText}`, 'error');
                }
            } catch (error) {
                showStatus(`Error: ${error.message}`, 'error');
            }
        });

        function showStatus(message, type) {
            statusDiv.textContent = message;
            statusDiv.className = `status ${type}`;
            statusDiv.style.display = 'block';

            if (type === 'success') {
                setTimeout(() => {
                    statusDiv.style.display = 'none';
                }, 5000);
            }
        }

        function fillDOI(doi) {
            doiInput.value = doi;
            doiInput.focus();
            // Add visual feedback
            doiInput.style.animation = 'none';
            setTimeout(() => {
                doiInput.style.animation = 'pulse 0.5s';
            }, 10);
        }

        function clearForm() {
            doiInput.value = '';
            statusDiv.style.display = 'none';
            doiInput.focus();
        }

        // Add pulse animation for input focus
        const style = document.createElement('style');
        style.textContent = `
            @keyframes pulse {
                0% { box-shadow: 0 0 0 0 rgba(102, 126, 234, 0.4); }
                70% { box-shadow: 0 0 0 10px rgba(102, 126, 234, 0); }
                100% { box-shadow: 0 0 0 0 rgba(102, 126, 234, 0); }
            }
        `;
        document.head.appendChild(style);

    </script>
</body>
</html>"##;

    let html = html
        .replace("{{SERVICE_NAME}}", SERVICE_NAME)
        .replace("{{SERVICE_TITLE}}", SERVICE_TITLE)
        .replace("{{SERVICE_DESCRIPTION}}", SERVICE_DESCRIPTION)
        .replace("{{EXAMPLE_DOIS}}", &examples_html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
