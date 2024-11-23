class Pomodoro {
    constructor() {
        this.timeLeft = 25 * 60; // 25 minutos em segundos
        this.isRunning = false;
        this.isBreak = false;
        this.completedSessions = 0;
        this.totalFocusTime = 0;
        this.timer = null;
        this.sound = null;
        this.tasks = JSON.parse(localStorage.getItem('pomodoro-tasks')) || [];
        this.dndMode = false;
        this.language = 'pt'; // Idioma padrão
        this.translations = {
            pt: {
                focus: 'Modo Foco',
                break: 'Modo Pausa',
                paused: 'PAUSADO',
                start: 'Iniciar',
                started: 'Iniciado',
                pause: 'Pausar',
                reset: 'Reiniciar',
                settings: 'Configurações',
                focusTime: 'Tempo de Foco (minutos):',
                breakTime: 'Tempo de Pausa (minutos):',
                ambientSound: 'Som Ambiente:',
                noSound: 'Sem som',
                forest: 'Floresta',
                cavern: 'Caverna',
                rain: 'Chuva',
                dndMode: 'Modo Não Perturbe',
                dndDescription: 'Bloqueia notificações do sistema durante o foco',
                statistics: 'Estatísticas',
                completedSessions: 'Sessões completadas:',
                totalFocusTime: 'Tempo total focado:',
                tasks: 'Lista de Tarefas',
                newTask: 'Nova tarefa...'
            },
            en: {
                focus: 'Focus Mode',
                break: 'Break Mode',
                paused: 'PAUSED',
                start: 'Start',
                started: 'Started',
                pause: 'Pause',
                reset: 'Reset',
                settings: 'Settings',
                focusTime: 'Focus Time (minutes):',
                breakTime: 'Break Time (minutes):',
                ambientSound: 'Ambient Sound:',
                noSound: 'No sound',
                forest: 'Forest',
                cavern: 'Cavern',
                rain: 'Rain',
                dndMode: 'Do Not Disturb',
                dndDescription: 'Blocks system notifications during focus',
                statistics: 'Statistics',
                completedSessions: 'Completed sessions:',
                totalFocusTime: 'Total focus time:',
                tasks: 'Task List',
                newTask: 'New task...'
            }
        };
        
        this.initializeElements();
        this.initializeEventListeners();
        this.setupNotifications();
        this.renderTasks();
    }

    initializeElements() {
        this.timeDisplay = document.getElementById('time-display');
        this.startButton = document.getElementById('start');
        this.pauseButton = document.getElementById('pause');
        this.resetButton = document.getElementById('reset');
        this.focusTimeInput = document.getElementById('focus-time');
        this.breakTimeInput = document.getElementById('break-time');
        this.ambientSoundSelect = document.getElementById('ambient-sound');
        this.volumeControl = document.getElementById('volume');
        this.completedSessionsDisplay = document.getElementById('completed-sessions');
        this.totalFocusTimeDisplay = document.getElementById('total-focus-time');
        this.newTaskInput = document.getElementById('new-task');
        this.addTaskButton = document.getElementById('add-task');
        this.tasksList = document.getElementById('tasks');
        this.dndToggle = document.getElementById('dnd-mode');
        
        this.timerCircle = document.querySelector('.timer-circle');
        this.timerMode = document.querySelector('.timer-mode');
        this.timerStatus = document.querySelector('.timer-status');
        this.languageButton = document.getElementById('toggle-language');
        
        this.notificationSound = new Audio('sounds/melodical.wav');
        this.notificationSound.volume = 0.5;
    }

    initializeEventListeners() {
        this.startButton.addEventListener('click', () => this.start());
        this.pauseButton.addEventListener('click', () => this.pause());
        this.resetButton.addEventListener('click', () => this.reset());
        this.ambientSoundSelect.addEventListener('change', () => this.changeAmbientSound());
        this.volumeControl.addEventListener('input', () => this.updateVolume());
        
        this.focusTimeInput.addEventListener('change', () => this.updateTimer());
        this.breakTimeInput.addEventListener('change', () => this.updateTimer());
        this.addTaskButton.addEventListener('click', () => this.addTask());
        this.newTaskInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') this.addTask();
        });
        this.dndToggle.addEventListener('change', () => this.toggleDndMode());
        this.languageButton.addEventListener('click', () => this.toggleLanguage());
    }

    async setupNotifications() {
        if ('Notification' in window) {
            const permission = await Notification.requestPermission();
            this.notificationsEnabled = permission === 'granted';
        }
    }

    start() {
        if (!this.isRunning) {
            this.isRunning = true;
            this.timer = setInterval(() => this.tick(), 1000);
            this.playAmbientSound();
            this.timerCircle.classList.add('active');
            this.timerStatus.classList.remove('paused');
            this.timerStatus.textContent = '';
            
            const t = this.translations[this.language];
            this.startButton.innerHTML = `<span class="btn-icon">▶</span>${t.started}`;
        }
    }

    pause() {
        if (this.isRunning) {
            this.isRunning = false;
            clearInterval(this.timer);
            this.pauseAmbientSound();
            this.timerCircle.classList.remove('active');
            
            const t = this.translations[this.language];
            this.startButton.innerHTML = `<span class="btn-icon">▶</span>${t.start}`;
            this.timerStatus.textContent = t.paused;
            this.timerStatus.classList.add('paused');
        }
    }

    reset() {
        this.pause();
        this.isBreak = false;
        this.timeLeft = this.focusTimeInput.value * 60;
        this.updateDisplay();
    }

    tick() {
        this.timeLeft--;
        
        if (this.timeLeft <= 0) {
            this.completeSession();
        }
        
        this.updateDisplay();
    }

    completeSession() {
        this.pause();
        
        this.playNotificationSound();
        
        if (!this.isBreak) {
            this.completedSessions++;
            this.totalFocusTime += parseInt(this.focusTimeInput.value);
            this.updateStatistics();
            this.showNotification('Tempo de foco concluído!', 'Hora de fazer uma pausa.');
            this.timeLeft = this.breakTimeInput.value * 60;
            this.isBreak = true;
        } else {
            this.showNotification('Pausa concluída!', 'Hora de voltar ao foco.');
            this.timeLeft = this.focusTimeInput.value * 60;
            this.isBreak = false;
        }
        this.updateDisplay();
    }

    updateDisplay() {
        const minutes = Math.floor(this.timeLeft / 60);
        const seconds = this.timeLeft % 60;
        this.timeDisplay.textContent = 
            `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        
        const t = this.translations[this.language];
        this.timerMode.textContent = this.isBreak ? t.break : t.focus;
        
        // Atualizar status de pausa
        if (!this.isRunning && this.timeLeft !== this.focusTimeInput.value * 60) {
            this.timerStatus.textContent = t.paused;
            this.timerStatus.classList.add('paused');
        } else {
            this.timerStatus.textContent = '';
            this.timerStatus.classList.remove('paused');
        }
    }

    updateStatistics() {
        this.completedSessionsDisplay.textContent = this.completedSessions;
        this.totalFocusTimeDisplay.textContent = this.totalFocusTime;
    }

    showNotification(title, body) {
        if (this.notificationsEnabled && !this.dndMode) {
            new Notification(title, { body });
        }
    }

    playAmbientSound() {
        const soundChoice = this.ambientSoundSelect.value;
        if (soundChoice !== 'none') {
            if (!this.sound) {
                this.sound = new Audio(`sounds/${soundChoice}.wav`);
                this.sound.loop = true;
            }
            this.sound.volume = this.volumeControl.value / 100;
            this.sound.play().catch(error => {
                console.log('Erro ao reproduzir o som:', error);
            });
        }
    }

    pauseAmbientSound() {
        if (this.sound) {
            this.sound.pause();
        }
    }

    updateVolume() {
        if (this.sound) {
            this.sound.volume = this.volumeControl.value / 100;
        }
    }

    changeAmbientSound() {
        this.pauseAmbientSound();
        this.sound = null;
        if (this.isRunning) {
            this.playAmbientSound();
        }
    }

    updateTimer() {
        if (!this.isRunning) {
            if (!this.isBreak) {
                this.timeLeft = this.focusTimeInput.value * 60;
            } else {
                this.timeLeft = this.breakTimeInput.value * 60;
            }
            this.updateDisplay();
        }
    }

    addTask() {
        const taskText = this.newTaskInput.value.trim();
        if (taskText) {
            const task = {
                id: Date.now(),
                text: taskText,
                completed: false
            };
            this.tasks.push(task);
            this.saveTasks();
            this.renderTasks();
            this.newTaskInput.value = '';
        }
    }

    renderTasks() {
        this.tasksList.innerHTML = '';
        this.tasks.forEach(task => {
            const li = document.createElement('li');
            li.className = `task-item ${task.completed ? 'completed' : ''}`;
            li.innerHTML = `
                <div class="task-checkbox ${task.completed ? 'checked' : ''}"
                     onclick="pomodoro.toggleTask(${task.id})"></div>
                <span>${task.text}</span>
                <button class="task-delete" onclick="pomodoro.deleteTask(${task.id})">×</button>
            `;
            this.tasksList.appendChild(li);
        });
    }

    toggleTask(id) {
        const task = this.tasks.find(t => t.id === id);
        if (task) {
            task.completed = !task.completed;
            this.saveTasks();
            this.renderTasks();
        }
    }

    deleteTask(id) {
        this.tasks = this.tasks.filter(t => t.id !== id);
        this.saveTasks();
        this.renderTasks();
    }

    saveTasks() {
        localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
    }

    toggleDndMode() {
        this.dndMode = this.dndToggle.checked;
        if (this.dndMode) {
            this.notificationsEnabled = false;
            this.notificationSound.volume = 0;
        } else {
            this.setupNotifications();
            this.notificationSound.volume = 0.5;
        }
    }

    playNotificationSound() {
        if (!this.dndMode) {
            this.notificationSound.currentTime = 0;
            this.notificationSound.play().catch(error => {
                console.log('Erro ao reproduzir o som de notificação:', error);
            });
        }
    }

    toggleLanguage() {
        this.language = this.language === 'pt' ? 'en' : 'pt';
        this.updateInterface();
        this.updateDisplay();
    }

    updateInterface() {
        const t = this.translations[this.language];
        
        // Atualizar textos dos botões
        if (this.isRunning) {
            this.startButton.innerHTML = `<span class="btn-icon">▶</span>${t.started}`;
        } else {
            this.startButton.innerHTML = `<span class="btn-icon">▶</span>${t.start}`;
        }
        this.pauseButton.innerHTML = `<span class="btn-icon">⏸</span>${t.pause}`;
        this.resetButton.innerHTML = `<span class="btn-icon">↺</span>${t.reset}`;
        
        // Atualizar títulos das seções
        const settingsTitle = document.querySelector('.settings h2');
        const statisticsTitle = document.querySelector('.statistics h2');
        const taskListTitle = document.querySelector('.task-list h2');
        
        if (settingsTitle) settingsTitle.textContent = t.settings;
        if (statisticsTitle) statisticsTitle.textContent = t.statistics;
        if (taskListTitle) taskListTitle.textContent = t.tasks;
        
        // Atualizar labels das configurações
        const timeLabels = document.querySelectorAll('.time-settings label');
        if (timeLabels[0]) timeLabels[0].childNodes[0].textContent = t.focusTime;
        if (timeLabels[1]) timeLabels[1].childNodes[0].textContent = t.breakTime;
        
        // Atualizar label do som
        const soundLabel = document.querySelector('.sound-settings label');
        if (soundLabel) soundLabel.childNodes[0].textContent = t.ambientSound;
        
        // Atualizar opções de som
        const soundSelect = this.ambientSoundSelect;
        if (soundSelect) {
            soundSelect.options[0].text = t.noSound;
            soundSelect.options[1].text = t.forest;
            soundSelect.options[2].text = t.cavern;
            soundSelect.options[3].text = t.rain;
        }
        
        // Atualizar modo não perturbe
        const dndLabel = document.querySelector('.dnd-label');
        const dndDesc = document.querySelector('.dnd-description');
        if (dndLabel) dndLabel.textContent = t.dndMode;
        if (dndDesc) dndDesc.textContent = t.dndDescription;
        
        // Atualizar estatísticas
        const stats = document.querySelectorAll('.statistics p');
        if (stats[0]) stats[0].textContent = `${t.completedSessions} ${this.completedSessions}`;
        if (stats[1]) stats[1].textContent = `${t.totalFocusTime} ${this.totalFocusTime} min`;
        
        // Atualizar placeholder da tarefa
        if (this.newTaskInput) this.newTaskInput.placeholder = t.newTask;
        
        // Atualizar modo e status
        if (this.timerMode) this.timerMode.textContent = this.isBreak ? t.break : t.focus;
        if (!this.isRunning && this.timeLeft !== this.focusTimeInput.value * 60) {
            if (this.timerStatus) {
                this.timerStatus.textContent = t.paused;
                this.timerStatus.classList.add('paused');
            }
        }
    }
}

// Inicializar o Pomodoro quando a página carregar
document.addEventListener('DOMContentLoaded', () => {
    window.pomodoro = new Pomodoro();
}); 