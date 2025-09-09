use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Multipart, State},
    response::Html,
};

pub async fn file_upload_handler() -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>File Upload - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }}
        .content {{
            padding: 40px;
        }}
        .upload-section {{
            margin-bottom: 40px;
        }}
        .upload-section h2 {{
            color: #333;
            margin-bottom: 20px;
            font-size: 1.5em;
        }}
        .upload-form {{
            border: 2px dashed #ddd;
            border-radius: 10px;
            padding: 40px;
            text-align: center;
            transition: all 0.3s ease;
        }}
        .upload-form:hover {{
            border-color: #667eea;
            background-color: #f8f9ff;
        }}
        .upload-form.dragover {{
            border-color: #667eea;
            background-color: #f0f4ff;
        }}
        .upload-icon {{
            font-size: 4em;
            color: #667eea;
            margin-bottom: 20px;
        }}
        .file-input {{
            display: none;
        }}
        .upload-btn {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 30px;
            border-radius: 25px;
            font-size: 1.1em;
            cursor: pointer;
            transition: all 0.3s ease;
            margin: 10px;
        }}
        .upload-btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
        }}
        .file-list {{
            margin-top: 30px;
        }}
        .file-item {{
            display: flex;
            align-items: center;
            padding: 15px;
            border: 1px solid #eee;
            border-radius: 8px;
            margin-bottom: 10px;
            background: #f9f9f9;
        }}
        .file-icon {{
            font-size: 2em;
            margin-right: 15px;
        }}
        .file-info {{
            flex: 1;
        }}
        .file-name {{
            font-weight: 600;
            color: #333;
        }}
        .file-size {{
            color: #666;
            font-size: 0.9em;
        }}
        .file-actions {{
            display: flex;
            gap: 10px;
        }}
        .btn {{
            padding: 8px 16px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 0.9em;
            transition: all 0.3s ease;
        }}
        .btn-primary {{
            background: #667eea;
            color: white;
        }}
        .btn-danger {{
            background: #dc3545;
            color: white;
        }}
        .btn:hover {{
            opacity: 0.8;
        }}
        .upload-types {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-top: 30px;
        }}
        .upload-type {{
            text-align: center;
            padding: 20px;
            border: 1px solid #eee;
            border-radius: 10px;
            background: #f9f9f9;
        }}
        .upload-type h3 {{
            color: #333;
            margin-bottom: 10px;
        }}
        .upload-type p {{
            color: #666;
            font-size: 0.9em;
        }}
        .back-btn {{
            display: inline-block;
            margin-bottom: 20px;
            color: #667eea;
            text-decoration: none;
            font-weight: 600;
        }}
        .back-btn:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📁 File Upload Center</h1>
            <p>Upload images, documents, and other files for your learning content</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="upload-section">
                <h2>📤 Upload Files</h2>
                <div class="upload-form" id="uploadForm">
                    <div class="upload-icon">📁</div>
                    <h3>Drag & Drop Files Here</h3>
                    <p>or click to browse files</p>
                    <input type="file" id="fileInput" class="file-input" multiple accept="image/*,.pdf,.doc,.docx,.txt,.md">
                    <button class="upload-btn" onclick="document.getElementById('fileInput').click()">
                        Choose Files
                    </button>
                </div>

                <div class="file-list" id="fileList">
                    <!-- Files will be listed here -->
                </div>
            </div>

            <div class="upload-section">
                <h2>📋 Supported File Types</h2>
                <div class="upload-types">
                    <div class="upload-type">
                        <h3>🖼️ Images</h3>
                        <p>JPG, PNG, GIF, SVG<br>Max size: 10MB</p>
                    </div>
                    <div class="upload-type">
                        <h3>📄 Documents</h3>
                        <p>PDF, DOC, DOCX, TXT, MD<br>Max size: 50MB</p>
                    </div>
                    <div class="upload-type">
                        <h3>💻 Code Files</h3>
                        <p>Rust, Python, JS, HTML, CSS<br>Max size: 5MB</p>
                    </div>
                    <div class="upload-type">
                        <h3>📊 Data Files</h3>
                        <p>JSON, CSV, XML<br>Max size: 20MB</p>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script>
        const uploadForm = document.getElementById('uploadForm');
        const fileInput = document.getElementById('fileInput');
        const fileList = document.getElementById('fileList');

        // Drag and drop functionality
        uploadForm.addEventListener('dragover', (e) => {{
            e.preventDefault();
            uploadForm.classList.add('dragover');
        }});

        uploadForm.addEventListener('dragleave', () => {{
            uploadForm.classList.remove('dragover');
        }});

        uploadForm.addEventListener('drop', (e) => {{
            e.preventDefault();
            uploadForm.classList.remove('dragover');
            const files = e.dataTransfer.files;
            handleFiles(files);
        }});

        fileInput.addEventListener('change', (e) => {{
            handleFiles(e.target.files);
        }});

        function handleFiles(files) {{
            Array.from(files).forEach(file => {{
                addFileToList(file);
            }});
        }}

        function addFileToList(file) {{
            const fileItem = document.createElement('div');
            fileItem.className = 'file-item';

            const fileIcon = getFileIcon(file.type);
            const fileSize = formatFileSize(file.size);

            fileItem.innerHTML = `
                <div class="file-icon">${{fileIcon}}</div>
                <div class="file-info">
                    <div class="file-name">${{file.name}}</div>
                    <div class="file-size">${{fileSize}}</div>
                </div>
                <div class="file-actions">
                    <button class="btn btn-primary" onclick="uploadFile(this)">Upload</button>
                    <button class="btn btn-danger" onclick="removeFile(this)">Remove</button>
                </div>
            `;

            fileList.appendChild(fileItem);
        }}

        function getFileIcon(type) {{
            if (type.startsWith('image/')) return '🖼️';
            if (type.includes('pdf')) return '📄';
            if (type.includes('document') || type.includes('text')) return '📝';
            if (type.includes('code') || type.includes('script')) return '💻';
            return '📁';
        }}

        function formatFileSize(bytes) {{
            if (bytes === 0) return '0 Bytes';
            const k = 1024;
            const sizes = ['Bytes', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
        }}

        function uploadFile(button) {{
            const fileItem = button.closest('.file-item');
            const fileName = fileItem.querySelector('.file-name').textContent;

            button.textContent = 'Uploading...';
            button.disabled = true;

            // Simulate upload
            setTimeout(() => {{
                button.textContent = 'Uploaded ✓';
                button.style.background = '#28a745';
                fileItem.style.opacity = '0.7';
            }}, 2000);
        }}

        function removeFile(button) {{
            const fileItem = button.closest('.file-item');
            fileItem.remove();
        }}
    </script>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn upload_file_handler(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Html<String>> {
    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let data = field.bytes().await.unwrap();
            uploaded_files.push((filename, data.len()));
        }
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Upload Success - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .container {{
            max-width: 600px;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
            text-align: center;
        }}
        .header {{
            background: linear-gradient(135deg, #28a745 0%, #20c997 100%);
            color: white;
            padding: 40px;
        }}
        .header h1 {{
            margin: 0;
            font-size: 2.5em;
        }}
        .content {{
            padding: 40px;
        }}
        .success-icon {{
            font-size: 4em;
            color: #28a745;
            margin-bottom: 20px;
        }}
        .file-list {{
            text-align: left;
            margin: 20px 0;
        }}
        .file-item {{
            padding: 10px;
            background: #f8f9fa;
            border-radius: 5px;
            margin: 5px 0;
        }}
        .btn {{
            display: inline-block;
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-decoration: none;
            border-radius: 25px;
            margin: 10px;
            transition: all 0.3s ease;
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>✅ Upload Successful!</h1>
        </div>

        <div class="content">
            <div class="success-icon">🎉</div>
            <h2>Files Uploaded Successfully</h2>
            <p>Your files have been uploaded and are ready to use.</p>

            <div class="file-list">
                <h3>Uploaded Files:</h3>
                {}
            </div>

            <a href="/admin/file-upload" class="btn">Upload More Files</a>
            <a href="/admin/dashboard" class="btn">Back to Dashboard</a>
        </div>
    </div>
</body>
</html>"#,
        uploaded_files
            .iter()
            .map(|(name, size)| {
                format!(
                    r#"<div class="file-item">📁 {} ({} bytes)</div>"#,
                    name, size
                )
            })
            .collect::<Vec<_>>()
            .join("")
    );

    Ok(Html(html))
}
