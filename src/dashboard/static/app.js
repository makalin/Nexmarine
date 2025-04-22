class NexmarineDashboard {
    constructor() {
        this.ws = null;
        this.messageCount = 0;
        this.lastMessageTime = Date.now();
        this.dataRateInterval = null;
        this.connect();
        this.setupEventListeners();
    }

    connect() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const host = window.location.hostname;
        this.ws = new WebSocket(`${protocol}//${host}:8000/ws`);

        this.ws.onopen = () => {
            console.log('Connected to Nexmarine server');
            this.updateConnectionStatus(true);
        };

        this.ws.onclose = () => {
            console.log('Disconnected from Nexmarine server');
            this.updateConnectionStatus(false);
            setTimeout(() => this.connect(), 5000);
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.updateConnectionStatus(false);
        };

        this.ws.onmessage = (event) => {
            this.handleMessage(event.data);
        };
    }

    setupEventListeners() {
        // Update data rate every second
        this.dataRateInterval = setInterval(() => {
            const now = Date.now();
            const rate = this.messageCount / ((now - this.lastMessageTime) / 1000);
            document.getElementById('data-rate').textContent = rate.toFixed(1);
            this.messageCount = 0;
            this.lastMessageTime = now;
        }, 1000);

        // Handle navigation between panels
        document.querySelectorAll('.sidebar a').forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const targetId = link.getAttribute('href').substring(1);
                this.showPanel(targetId);
            });
        });
    }

    showPanel(panelId) {
        // Hide all panels
        document.querySelectorAll('.panel').forEach(panel => {
            panel.style.display = 'none';
        });

        // Show selected panel
        const panel = document.getElementById(panelId);
        if (panel) {
            panel.style.display = 'block';
        }

        // Update active state in sidebar
        document.querySelectorAll('.sidebar a').forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === `#${panelId}`) {
                link.classList.add('active');
            }
        });
    }

    updateConnectionStatus(connected) {
        const statusDot = document.querySelector('.status-dot');
        const statusText = document.querySelector('.status-text');

        if (connected) {
            statusDot.style.backgroundColor = 'var(--success-color)';
            statusText.textContent = 'Connected';
        } else {
            statusDot.style.backgroundColor = 'var(--error-color)';
            statusText.textContent = 'Disconnected';
        }
    }

    handleMessage(data) {
        try {
            const message = JSON.parse(data);
            this.messageCount++;
            document.getElementById('last-update').textContent = new Date().toLocaleTimeString();

            switch (message.path) {
                case 'nav.position.latitude':
                    this.updateLatitude(message.value);
                    break;
                case 'nav.position.longitude':
                    this.updateLongitude(message.value);
                    break;
                case 'nav.speed.knots':
                    this.updateSpeed(message.value);
                    break;
                case 'nav.course':
                    this.updateCourse(message.value);
                    break;
                case 'nav.depth':
                    this.updateDepth(message.value);
                    break;
            }
        } catch (error) {
            console.error('Error parsing message:', error);
        }
    }

    updateLatitude(value) {
        const lat = this.formatCoordinate(value, 'N', 'S');
        document.getElementById('latitude').textContent = lat;
    }

    updateLongitude(value) {
        const lon = this.formatCoordinate(value, 'E', 'W');
        document.getElementById('longitude').textContent = lon;
    }

    updateSpeed(value) {
        document.getElementById('sog').textContent = `${value.toFixed(1)} kts`;
    }

    updateCourse(value) {
        document.getElementById('cog').textContent = `${value.toFixed(0)}°`;
    }

    updateDepth(value) {
        document.getElementById('depth').textContent = `${value.toFixed(1)} m`;
    }

    formatCoordinate(value, posHemisphere, negHemisphere) {
        const hemisphere = value >= 0 ? posHemisphere : negHemisphere;
        const absValue = Math.abs(value);
        const degrees = Math.floor(absValue);
        const minutes = Math.floor((absValue - degrees) * 60);
        const seconds = Math.floor(((absValue - degrees) * 60 - minutes) * 60);
        return `${degrees}°${minutes}'${seconds}"${hemisphere}`;
    }
}

// Initialize dashboard when the page loads
document.addEventListener('DOMContentLoaded', () => {
    window.dashboard = new NexmarineDashboard();
}); 