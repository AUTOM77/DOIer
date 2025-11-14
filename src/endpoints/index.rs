use actix_web::{get, HttpResponse, Responder};
use crate::config::constant::{EXAMPLE_DOIS, SERVICE_NAME, SERVICE_DESCRIPTION};

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
    <title>DOI Paper Downloader - Instant Research Paper Access</title>
    <meta name="description" content="Download research papers instantly using DOI. Fast, reliable, and free academic paper downloader.">
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
            padding: 3rem;
            max-width: 700px;
            margin: 0 auto 4rem;
            box-shadow:
                0 30px 60px rgba(0, 0, 0, 0.12),
                0 10px 20px rgba(0, 0, 0, 0.08);
            border: 1px solid rgba(255, 255, 255, 0.5);
            animation: fadeInUp 1.2s ease;
        }

        .search-header {
            display: flex;
            align-items: center;
            gap: 1rem;
            margin-bottom: 2rem;
        }

        .search-icon {
            width: 48px;
            height: 48px;
            background: linear-gradient(135deg, var(--primary), var(--secondary));
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
        }

        .search-title {
            flex: 1;
        }

        .search-title h2 {
            font-size: 1.5rem;
            margin-bottom: 0.25rem;
        }

        .search-title p {
            color: var(--gray-600);
            font-size: 0.95rem;
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

        /* Features Section */
        .features {
            padding: 4rem 2rem;
            max-width: 1400px;
            margin: 0 auto;
        }

        .section-title {
            text-align: center;
            margin-bottom: 4rem;
        }

        .section-title h2 {
            font-size: clamp(2rem, 4vw, 3rem);
            margin-bottom: 1rem;
            color: white;
        }

        .section-title p {
            color: rgba(255, 255, 255, 0.8);
            font-size: 1.2rem;
        }

        .features-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 2rem;
            margin-bottom: 4rem;
        }

        .feature-card {
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(10px);
            border-radius: 20px;
            padding: 2.5rem;
            transition: all 0.4s;
            border: 1px solid rgba(255, 255, 255, 0.3);
            position: relative;
            overflow: hidden;
        }

        .feature-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 3px;
            background: linear-gradient(90deg, var(--primary), var(--secondary));
            transform: scaleX(0);
            transition: transform 0.3s;
        }

        .feature-card:hover {
            transform: translateY(-8px);
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
        }

        .feature-card:hover::before {
            transform: scaleX(1);
        }

        .feature-icon {
            width: 64px;
            height: 64px;
            background: linear-gradient(135deg, var(--primary), var(--secondary));
            border-radius: 16px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.8rem;
            margin-bottom: 1.5rem;
        }

        .feature-card h3 {
            font-size: 1.4rem;
            margin-bottom: 1rem;
            color: var(--gray-900);
        }

        .feature-card p {
            color: var(--gray-600);
            line-height: 1.8;
        }

        /* Statistics Section */
        .stats {
            padding: 4rem 2rem;
            text-align: center;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 2rem;
            max-width: 1000px;
            margin: 0 auto;
        }

        .stat-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 2rem;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }

        .stat-number {
            font-size: 2.5rem;
            font-weight: 800;
            color: white;
            margin-bottom: 0.5rem;
        }

        .stat-label {
            color: rgba(255, 255, 255, 0.8);
            font-size: 1.1rem;
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
            <div class="logo">DOI Paper Downloader</div>
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
                <h1>Access Research Papers Instantly</h1>
                <p class="hero-subtitle">Transform any DOI into a downloadable PDF in seconds. Simple, fast, and reliable.</p>

                <!-- Search Card -->
                <div class="search-card">
                    <div class="search-header">
                        <div class="search-icon">🔍</div>
                        <div class="search-title">
                            <h2>Download Papers</h2>
                            <p>Enter a DOI to get started</p>
                        </div>
                    </div>

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
                                <span>Download PDF</span>
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

        <!-- Features Section -->
        <section class="features" id="features">
            <div class="section-title">
                <h2>Why Choose Our Service?</h2>
                <p>Fast, reliable, and designed for researchers</p>
            </div>

            <div class="features-grid">
                <div class="feature-card">
                    <div class="feature-icon">⚡</div>
                    <h3>Lightning Fast</h3>
                    <p>Get your papers in seconds. Our optimized infrastructure ensures rapid downloads without compromising quality.</p>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🔒</div>
                    <h3>Secure & Private</h3>
                    <p>Your searches are private. We don't track or store your download history, ensuring complete confidentiality.</p>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🌍</div>
                    <h3>Global Access</h3>
                    <p>Access research from publishers worldwide. Supporting all major academic databases and repositories.</p>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🔧</div>
                    <h3>Developer Friendly</h3>
                    <p>Simple REST API for seamless integration. Build your own tools with our robust and well-documented endpoints.</p>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">📱</div>
                    <h3>Mobile Ready</h3>
                    <p>Works perfectly on all devices. Download papers on the go from your phone, tablet, or desktop.</p>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">💯</div>
                    <h3>Free Forever</h3>
                    <p>No hidden fees, no subscriptions. Access to knowledge should be universal and unrestricted.</p>
                </div>
            </div>
        </section>

        <!-- Statistics -->
        <section class="stats">
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-number">1M+</div>
                    <div class="stat-label">Papers Downloaded</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">99.9%</div>
                    <div class="stat-label">Uptime</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">< 2s</div>
                    <div class="stat-label">Average Response</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">150+</div>
                    <div class="stat-label">Publishers</div>
                </div>
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
            <p id="copyright">© 2024 DOI Paper Downloader. Built with passion for open science.</p>
        </div>
    </footer>

    <script>
        // Update copyright year
        document.getElementById('copyright').textContent = `© ${new Date().getFullYear()} DOI Paper Downloader. Built with passion for open science.`;

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

        // Intersection Observer for fade-in animations
        const observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -100px 0px'
        };

        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.style.opacity = '1';
                    entry.target.style.transform = 'translateY(0)';
                }
            });
        }, observerOptions);

        // Observe feature cards
        document.querySelectorAll('.feature-card').forEach(card => {
            card.style.opacity = '0';
            card.style.transform = 'translateY(30px)';
            card.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
            observer.observe(card);
        });

        // Observe stat cards
        document.querySelectorAll('.stat-card').forEach(card => {
            card.style.opacity = '0';
            card.style.transform = 'translateY(20px)';
            card.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
            observer.observe(card);
        });

        // Add counter animation for statistics
        function animateValue(element, start, end, duration) {
            let startTimestamp = null;
            const step = (timestamp) => {
                if (!startTimestamp) startTimestamp = timestamp;
                const progress = Math.min((timestamp - startTimestamp) / duration, 1);
                element.textContent = element.dataset.prefix +
                    Math.floor(progress * (end - start) + start) +
                    element.dataset.suffix;
                if (progress < 1) {
                    window.requestAnimationFrame(step);
                }
            };
            window.requestAnimationFrame(step);
        }

        // Animate statistics when they come into view
        const statsObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting && !entry.target.dataset.animated) {
                    entry.target.dataset.animated = 'true';
                    const statNumbers = entry.target.querySelectorAll('.stat-number');
                    statNumbers.forEach(stat => {
                        const text = stat.textContent;
                        if (text.includes('1M+')) {
                            stat.dataset.prefix = '';
                            stat.dataset.suffix = 'M+';
                            animateValue(stat, 0, 1, 1500);
                        } else if (text.includes('99.9%')) {
                            stat.dataset.prefix = '';
                            stat.dataset.suffix = '%';
                            animateValue(stat, 0, 99.9, 1500);
                        } else if (text.includes('150+')) {
                            stat.dataset.prefix = '';
                            stat.dataset.suffix = '+';
                            animateValue(stat, 0, 150, 1500);
                        }
                    });
                }
            });
        }, { threshold: 0.5 });

        const statsSection = document.querySelector('.stats');
        if (statsSection) {
            statsObserver.observe(statsSection);
        }
    </script>
</body>
</html>"##;

    let html = html
        .replace("{{SERVICE_NAME}}", SERVICE_NAME)
        .replace("{{SERVICE_DESCRIPTION}}", SERVICE_DESCRIPTION)
        .replace("{{EXAMPLE_DOIS}}", &examples_html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
