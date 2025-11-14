use actix_web::{get, HttpResponse, Responder};
use crate::config::constant::{EXAMPLE_DOIS, SERVICE_NAME, SERVICE_DESCRIPTION};

#[get("/")]
async fn index() -> impl Responder {
    let examples_html = EXAMPLE_DOIS
        .iter()
        .map(|doi| format!(r#"<span class="example-item" onclick="fillDOI('{}')">{}</span>"#, doi, doi))
        .collect::<Vec<_>>()
        .join("\n            ");

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DOI Paper Downloader</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }

        .container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            padding: 40px;
            max-width: 600px;
            width: 100%;
        }

        h1 {
            color: #333;
            margin-bottom: 10px;
            font-size: 2.5em;
            text-align: center;
        }

        .subtitle {
            color: #666;
            text-align: center;
            margin-bottom: 30px;
            font-size: 1.1em;
        }

        .info-box {
            background: #f8f9fa;
            border-left: 4px solid #667eea;
            padding: 15px;
            margin-bottom: 30px;
            border-radius: 4px;
        }

        .info-box p {
            color: #555;
            line-height: 1.6;
            margin-bottom: 10px;
        }

        .info-box code {
            background: #e9ecef;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
            color: #d63384;
        }

        .form-group {
            margin-bottom: 20px;
        }

        label {
            display: block;
            margin-bottom: 8px;
            color: #333;
            font-weight: 600;
        }

        input[type="text"] {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-size: 16px;
            transition: border-color 0.3s;
        }

        input[type="text"]:focus {
            outline: none;
            border-color: #667eea;
        }

        .button-group {
            display: flex;
            gap: 10px;
        }

        button {
            flex: 1;
            padding: 14px 24px;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s;
        }

        .btn-primary {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }

        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
        }

        .btn-secondary {
            background: #f8f9fa;
            color: #333;
            border: 2px solid #e0e0e0;
        }

        .btn-secondary:hover {
            background: #e9ecef;
        }

        .examples {
            margin-top: 20px;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 8px;
        }

        .examples h3 {
            color: #333;
            margin-bottom: 10px;
            font-size: 0.9em;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .example-item {
            display: inline-block;
            padding: 6px 12px;
            margin: 4px;
            background: white;
            border: 1px solid #dee2e6;
            border-radius: 4px;
            font-size: 0.85em;
            cursor: pointer;
            transition: all 0.2s;
        }

        .example-item:hover {
            background: #667eea;
            color: white;
            border-color: #667eea;
        }

        .status {
            margin-top: 20px;
            padding: 12px;
            border-radius: 8px;
            display: none;
            animation: slideIn 0.3s;
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
            background: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
        }

        .status.error {
            background: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }

        .status.info {
            background: #d1ecf1;
            color: #0c5460;
            border: 1px solid #bee5eb;
        }

        .api-endpoint {
            margin-top: 30px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 8px;
            border: 1px solid #e0e0e0;
        }

        .api-endpoint h3 {
            color: #333;
            margin-bottom: 10px;
        }

        .api-endpoint pre {
            background: #2d2d2d;
            color: #f8f8f2;
            padding: 15px;
            border-radius: 4px;
            overflow-x: auto;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 {{SERVICE_NAME}}</h1>
        <p class="subtitle">{{SERVICE_DESCRIPTION}}</p>

        <div class="info-box">
            <p><strong>What is this?</strong></p>
            <p>Enter a DOI (Digital Object Identifier) to download the corresponding research paper PDF directly.</p>
        </div>

        <form id="doiForm">
            <div class="form-group">
                <label for="doiInput">DOI (e.g., 10.1038/nature12373)</label>
                <input
                    type="text"
                    id="doiInput"
                    placeholder="10.xxxx/xxxxx"
                    autocomplete="off"
                />
            </div>

            <div class="button-group">
                <button type="submit" class="btn-primary">Download PDF</button>
                <button type="button" class="btn-secondary" onclick="clearForm()">Clear</button>
            </div>
        </form>

        <div id="status" class="status"></div>

        <div class="examples">
            <h3>Example DOIs (Click to try)</h3>
            {{EXAMPLE_DOIS}}
        </div>

        <div class="api-endpoint">
            <h3>API Endpoint</h3>
            <pre>GET /v1/doi/{doi}</pre>
            <p style="margin-top: 10px; color: #666; font-size: 0.9em;">
                Example: <code style="background: #e9ecef; padding: 2px 6px; border-radius: 3px;">/v1/doi/10.1038/nature12373</code>
            </p>
        </div>
    </div>

    <script>
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
        }

        function clearForm() {
            doiInput.value = '';
            statusDiv.style.display = 'none';
            doiInput.focus();
        }
    </script>
</body>
</html>"#;

    let html = html
        .replace("{{SERVICE_NAME}}", SERVICE_NAME)
        .replace("{{SERVICE_DESCRIPTION}}", SERVICE_DESCRIPTION)
        .replace("{{EXAMPLE_DOIS}}", &examples_html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
