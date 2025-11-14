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

        /* Progress Bar */
        .progress-bar-container {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 3px;
            background: rgba(255, 255, 255, 0.1);
            z-index: 9999;
            opacity: 0;
            transition: opacity 0.3s;
        }

        .progress-bar-container.active {
            opacity: 1;
        }

        .progress-bar {
            height: 100%;
            background: linear-gradient(90deg, var(--primary), var(--secondary));
            width: 0;
            transition: width 0.3s ease;
            box-shadow: 0 0 10px rgba(102, 126, 234, 0.5);
        }

        /* Toast Notifications */
        .toast-container {
            position: fixed;
            top: 80px;
            right: 20px;
            z-index: 9998;
            pointer-events: none;
        }

        .toast {
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px);
            border-radius: 12px;
            padding: 1rem 1.5rem;
            margin-bottom: 1rem;
            min-width: 300px;
            max-width: 400px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.3);
            display: flex;
            align-items: center;
            gap: 1rem;
            opacity: 0;
            transform: translateX(400px);
            animation: slideInFromRight 0.4s forwards;
            pointer-events: all;
        }

        @keyframes slideInFromRight {
            to {
                opacity: 1;
                transform: translateX(0);
            }
        }

        @keyframes slideOutToRight {
            from {
                opacity: 1;
                transform: translateX(0);
            }
            to {
                opacity: 0;
                transform: translateX(400px);
            }
        }

        .toast.hiding {
            animation: slideOutToRight 0.4s forwards;
        }

        .toast-icon {
            font-size: 1.5rem;
            flex-shrink: 0;
        }

        .toast-message {
            flex: 1;
            font-weight: 500;
            color: var(--gray-900);
        }

        .toast.success .toast-icon {
            color: #00b894;
        }

        .toast.error .toast-icon {
            color: #ff6b6b;
        }

        .toast.info .toast-icon {
            color: #74b9ff;
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
            background: rgba(255, 255, 255, 0.08);
            backdrop-filter: blur(24px);
            -webkit-backdrop-filter: blur(24px);
            border-radius: 20px;
            padding: 3rem;
            max-width: 1000px;
            margin: 0 auto 4rem;
            box-shadow:
                0 20px 40px rgba(0, 0, 0, 0.1),
                inset 0 1px 0 rgba(255, 255, 255, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.15);
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
            color: rgba(255, 255, 255, 0.7);
            font-size: 1.5rem;
            z-index: 1;
        }

        input[type="text"] {
            width: 100%;
            padding: 1.75rem 1.75rem 1.75rem 4rem;
            border: 2px solid rgba(255, 255, 255, 0.2);
            border-radius: 16px;
            font-size: 1.25rem;
            transition: all 0.3s;
            background: rgba(255, 255, 255, 0.08);
            backdrop-filter: blur(10px);
            box-shadow:
                0 4px 20px rgba(0, 0, 0, 0.08),
                inset 0 1px 0 rgba(255, 255, 255, 0.1);
            font-weight: 500;
            color: white;
        }

        input[type="text"]:focus {
            outline: none;
            border-color: rgba(102, 126, 234, 0.5);
            background: rgba(255, 255, 255, 0.12);
            box-shadow:
                0 0 0 4px rgba(102, 126, 234, 0.1),
                0 8px 30px rgba(102, 126, 234, 0.15),
                inset 0 1px 0 rgba(255, 255, 255, 0.2);
            transform: translateY(-1px);
        }

        input[type="text"]::placeholder {
            color: rgba(255, 255, 255, 0.5);
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
            background: linear-gradient(135deg, rgba(102, 126, 234, 0.9), rgba(118, 75, 162, 0.9));
            color: white;
            box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
        }

        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(102, 126, 234, 0.4);
            background: linear-gradient(135deg, var(--primary), var(--secondary));
        }

        .btn-secondary {
            background: rgba(255, 255, 255, 0.1);
            color: rgba(255, 255, 255, 0.9);
            border: 2px solid rgba(255, 255, 255, 0.2);
            backdrop-filter: blur(10px);
        }

        .btn-secondary:hover {
            background: rgba(255, 255, 255, 0.15);
            border-color: rgba(255, 255, 255, 0.3);
            box-shadow: 0 4px 15px rgba(255, 255, 255, 0.1);
        }

        /* Examples Section - Inline Version */
        .examples-inline {
            margin: 1rem 0 1.5rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }

        .examples-label {
            color: rgba(255, 255, 255, 0.6);
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
            padding: 0.25rem 0.6rem;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 6px;
            font-size: 0.7rem;
            cursor: pointer;
            transition: all 0.2s;
            border: 1px solid rgba(255, 255, 255, 0.15);
            font-weight: 500;
            color: rgba(255, 255, 255, 0.8);
            font-family: 'Courier New', monospace;
            white-space: nowrap;
            line-height: 1.2;
            backdrop-filter: blur(10px);
        }

        .example-item:hover {
            background: rgba(102, 126, 234, 0.3);
            color: white;
            border-color: rgba(102, 126, 234, 0.4);
            transform: translateY(-1px);
            box-shadow: 0 2px 8px rgba(102, 126, 234, 0.2);
        }

        /* API Section */
        .api-section {
            padding: 4rem 2rem;
            max-width: 1000px;
            margin: 0 auto;
        }

        .api-card {
            background: rgba(255, 255, 255, 0.08);
            backdrop-filter: blur(24px);
            -webkit-backdrop-filter: blur(24px);
            border-radius: 20px;
            padding: 3rem;
            box-shadow:
                0 20px 40px rgba(0, 0, 0, 0.1),
                inset 0 1px 0 rgba(255, 255, 255, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.15);
        }

        .api-card h3 {
            font-size: 1.8rem;
            margin-bottom: 2rem;
            color: white;
        }

        .api-card p {
            color: rgba(255, 255, 255, 0.8) !important;
        }

        .code-block {
            background: rgba(0, 0, 0, 0.4);
            color: #a6e3a1;
            padding: 1.5rem;
            border-radius: 12px;
            font-family: 'Courier New', monospace;
            margin-bottom: 1.5rem;
            position: relative;
            overflow-x: auto;
            border: 1px solid rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
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
    <!-- Progress Bar -->
    <div class="progress-bar-container" id="progressBarContainer">
        <div class="progress-bar" id="progressBar"></div>
    </div>

    <!-- Toast Container -->
    <div class="toast-container" id="toastContainer"></div>

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

        // Notification System
        function showToast(message, type) {
            const toastContainer = document.getElementById('toastContainer');
            const toast = document.createElement('div');
            toast.className = `toast ${type}`;

            const icons = {
                success: '✅',
                error: '❌',
                info: 'ℹ️'
            };

            toast.innerHTML = `
                <div class="toast-icon">${icons[type]}</div>
                <div class="toast-message">${message}</div>
            `;

            toastContainer.appendChild(toast);

            // Auto remove after 5 seconds
            setTimeout(() => {
                toast.classList.add('hiding');
                setTimeout(() => {
                    toast.remove();
                }, 400);
            }, 5000);
        }

        // Progress Bar
        function showProgress(percent) {
            const progressBarContainer = document.getElementById('progressBarContainer');
            const progressBar = document.getElementById('progressBar');

            if (percent === 0) {
                progressBarContainer.classList.remove('active');
                progressBar.style.width = '0%';
            } else if (percent === 100) {
                progressBar.style.width = '100%';
                setTimeout(() => {
                    progressBarContainer.classList.remove('active');
                    setTimeout(() => {
                        progressBar.style.width = '0%';
                    }, 300);
                }, 500);
            } else {
                progressBarContainer.classList.add('active');
                progressBar.style.width = percent + '%';
            }
        }

        // Form handling
        const doiForm = document.getElementById('doiForm');
        const doiInput = document.getElementById('doiInput');

        doiForm.addEventListener('submit', async (e) => {
            e.preventDefault();

            const doi = doiInput.value.trim();

            if (!doi) {
                showToast('Please enter a DOI', 'error');
                return;
            }

            if (!doi.startsWith('10.') || !doi.includes('/')) {
                showToast('Invalid DOI format. Must start with "10." and contain "/"', 'error');
                return;
            }

            showToast('Fetching paper...', 'info');
            showProgress(30);

            try {
                const url = `/v1/doi/${doi}`;
                showProgress(60);
                const response = await fetch(url);
                showProgress(80);

                if (response.ok) {
                    const blob = await response.blob();
                    showProgress(90);
                    const downloadUrl = window.URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = downloadUrl;
                    a.download = `${doi.replace(/\//g, '_')}.pdf`;
                    document.body.appendChild(a);
                    a.click();
                    window.URL.revokeObjectURL(downloadUrl);
                    document.body.removeChild(a);
                    showProgress(100);
                    showToast('PDF downloaded successfully!', 'success');
                } else {
                    showProgress(0);
                    const errorText = await response.text();
                    showToast(`Error: ${errorText || response.statusText}`, 'error');
                }
            } catch (error) {
                showProgress(0);
                showToast(`Error: ${error.message}`, 'error');
            }
        });

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
            doiInput.focus();
            showProgress(0);
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
