export interface Task {
  id: string;
  text: string;
  completed: boolean;
}

export interface SessionData {
  focusInterval: number;
  breakInterval: number;
  taskCompletionStatus: 'completed' | 'abandoned';
}

export interface ProductivityData {
  tasksCompleted: number;
  focusSessions: number;
  totalFocusTime: number; // in minutes
}

export type TimerMode = 'focus' | 'shortBreak' | 'longBreak';

export type Sound = 'forest' | 'cavern' | 'rain' | 'none';
