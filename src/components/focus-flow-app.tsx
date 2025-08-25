"use client";

import { useState, useEffect, useCallback, useMemo, useRef } from "react";
import type { Task, SessionData, ProductivityData, TimerMode, Sound } from "@/lib/types";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger, DialogDescription, DialogClose } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { Progress } from "@/components/ui/progress";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, ResponsiveContainer } from "recharts";
import {
  Play, Pause, RefreshCcw, SkipForward, Settings, ListChecks, BarChart2, Wind, Droplets, Leaf, VolumeX, Maximize, Minimize, Pencil, Trash2, X, Check as CheckIcon
} from "lucide-react";
import { cn } from "@/lib/utils";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";

function useLocalStorage<T>(key: string, initialValue: T): [T, (value: T | ((val: T) => T)) => void] {
    const [storedValue, setStoredValue] = useState<T>(() => {
        if (typeof window === 'undefined') {
            return initialValue;
        }
        try {
            const item = window.localStorage.getItem(key);
            return item ? JSON.parse(item) : initialValue;
        } catch (error) {
            console.error(error);
            return initialValue;
        }
    });

    const setValue = (value: T | ((val: T) => T)) => {
        try {
            const valueToStore = value instanceof Function ? value(storedValue) : value;
            setStoredValue(valueToStore);
            if (typeof window !== 'undefined') {
                window.localStorage.setItem(key, JSON.stringify(valueToStore));
            }
        } catch (error) {
            console.log(error);
        }
    };
    return [storedValue, setValue];
}

export function FocusFlowApp() {
  const [tasks, setTasks] = useLocalStorage<Task[]>("tasks", []);
  const [sessions, setSessions] = useLocalStorage<SessionData[]>("sessions", []);
  const [productivity, setProductivity] = useLocalStorage<ProductivityData>("productivity", {
    tasksCompleted: 0,
    focusSessions: 0,
    totalFocusTime: 0,
  });

  const [focusDuration, setFocusDuration] = useLocalStorage("focusDuration", 25);
  const [shortBreakDuration, setShortBreakDuration] = useLocalStorage("shortBreakDuration", 5);
  const [longBreakDuration, setLongBreakDuration] = useLocalStorage("longBreakDuration", 15);
  const [autoStart, setAutoStart] = useLocalStorage("autoStart", true);
  
  const [mode, setMode] = useState<TimerMode>("focus");
  const [isActive, setIsActive] = useState(false);
  const [secondsLeft, setSecondsLeft] = useState(focusDuration * 60);
  const [newTaskText, setNewTaskText] = useState("");
  const [selectedSound, setSelectedSound] = useLocalStorage<Sound>("sound", "none");
  const [volume, setVolume] = useLocalStorage("volume", 0.3);
  const [isFullScreen, setIsFullScreen] = useState(false);
  const [isMounted, setIsMounted] = useState(false);
  const [editingTaskId, setEditingTaskId] = useState<string | null>(null);
  const [editingTaskText, setEditingTaskText] = useState('');

  const audioRefs = {
    forest: useRef<HTMLAudioElement>(null),
    cavern: useRef<HTMLAudioElement>(null),
    rain: useRef<HTMLAudioElement>(null),
    notification: useRef<HTMLAudioElement>(null),
  };

  useEffect(() => {
    setIsMounted(true);
  }, []);
  
  const durations: { [key in TimerMode]: number } = useMemo(() => ({
    focus: focusDuration,
    shortBreak: shortBreakDuration,
    longBreak: longBreakDuration,
  }), [focusDuration, shortBreakDuration, longBreakDuration]);

  const recordSession = useCallback((status: 'completed' | 'abandoned') => {
    const newSession: SessionData = {
      focusInterval: durations[mode],
      breakInterval: mode === 'focus' ? durations.shortBreak : durations.focus,
      taskCompletionStatus: status,
    };
    setSessions(prev => [...prev, newSession]);
    
    if (mode === 'focus') {
      setProductivity(prev => ({
        ...prev,
        focusSessions: prev.focusSessions + 1,
        totalFocusTime: prev.totalFocusTime + durations[mode],
      }));
    }
  }, [mode, durations, setSessions, setProductivity]);

  const handleModeChange = useCallback((newMode: TimerMode) => {
    setMode(newMode);
    setIsActive(false);
    setSecondsLeft(durations[newMode] * 60);
  }, [durations]);

  const [isAudioEnabled, setIsAudioEnabled] = useState(false);

  const enableAudio = useCallback(() => {
    if (!isAudioEnabled) {
      setIsAudioEnabled(true);
      Object.values(audioRefs).forEach(ref => {
        if (ref.current) {
          ref.current.load();
        }
      });
    }
  }, [isAudioEnabled, audioRefs.notification, audioRefs.rain, audioRefs.forest, audioRefs.cavern]);

  const handleToggleActive = () => {
    enableAudio();
    setIsActive(!isActive);
  };

  const handleReset = () => {
    setIsActive(false);
    setSecondsLeft(durations[mode] * 60);
  };
  
  const playNotificationSound = useCallback(() => {
    Object.values(audioRefs).forEach(ref => {
      if (ref.current && ref.current !== audioRefs.notification.current) {
        ref.current.pause();
      }
    });

    if (audioRefs.notification?.current) {
      audioRefs.notification.current.currentTime = 0;
      audioRefs.notification.current.play()
        .catch(() => {
          playBeepSound();
        });
    } else {
      playBeepSound();
    }
  }, [audioRefs.notification, audioRefs.rain, audioRefs.forest, audioRefs.cavern]);

  const playBeepSound = useCallback(() => {
    try {
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const oscillator = audioContext.createOscillator();
      const gainNode = audioContext.createGain();
      
      oscillator.connect(gainNode);
      gainNode.connect(audioContext.destination);
      
      oscillator.frequency.setValueAtTime(800, audioContext.currentTime);
      oscillator.type = 'sine';
      
      gainNode.gain.setValueAtTime(0, audioContext.currentTime);
      gainNode.gain.linearRampToValueAtTime(0.3, audioContext.currentTime + 0.01);
      gainNode.gain.exponentialRampToValueAtTime(0.001, audioContext.currentTime + 0.5);
      
      oscillator.start(audioContext.currentTime);
      oscillator.stop(audioContext.currentTime + 0.5);
    } catch (error) {
      console.warn('Web Audio API not supported:', error);
    }
  }, []);

  const handleSkip = useCallback(() => {
    playNotificationSound();
    recordSession('completed');
    const newMode: TimerMode = mode === 'focus' ? 'shortBreak' : 'focus';
    handleModeChange(newMode);
    if(autoStart) {
        setIsActive(true);
    } else {
        setIsActive(false);
    }
  }, [mode, handleModeChange, recordSession, playNotificationSound, autoStart]);

  const handleAddTask = (e: React.FormEvent) => {
    e.preventDefault();
    if (newTaskText.trim()) {
      setTasks([...tasks, { id: crypto.randomUUID(), text: newTaskText.trim(), completed: false }]);
      setNewTaskText("");
    }
  };

  const handleSoundChange = useCallback((sound: Sound) => {
    enableAudio();
    setSelectedSound(sound);
  }, [enableAudio, setSelectedSound]);

  const handleToggleTask = (id: string) => {
    const newTasks = tasks.map(task => {
        if (task.id === id) {
            if(!task.completed) {
                setProductivity(p => ({...p, tasksCompleted: p.tasksCompleted + 1}));
            } else { 
                setProductivity(p => ({...p, tasksCompleted: Math.max(0, p.tasksCompleted - 1)}));
            }
            return { ...task, completed: !task.completed };
        }
        return task;
    });
    setTasks(newTasks);
  };
  
  const handleDeleteTask = (id: string) => {
      const taskToDelete = tasks.find(task => task.id === id);
      if (taskToDelete && taskToDelete.completed) {
        setProductivity(p => ({...p, tasksCompleted: Math.max(0, p.tasksCompleted - 1)}));
      }
      setTasks(tasks.filter(task => task.id !== id));
  };
  
  const handleEditTask = (task: Task) => {
      setEditingTaskId(task.id);
      setEditingTaskText(task.text);
  };

  const handleUpdateTask = (e: React.FormEvent) => {
      e.preventDefault();
      if (editingTaskText.trim()) {
          setTasks(tasks.map(task => 
              task.id === editingTaskId ? { ...task, text: editingTaskText.trim() } : task
          ));
          setEditingTaskId(null);
          setEditingTaskText('');
      }
  };
  
  const toggleFullScreen = useCallback(() => {
    if (typeof document === 'undefined') return;
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
      setIsFullScreen(true);
    } else {
      if (document.exitFullscreen) {
        document.exitFullscreen();
        setIsFullScreen(false);
      }
    }
  }, []);

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;

    if (isActive && secondsLeft > 0) {
      interval = setInterval(() => {
        setSecondsLeft((seconds) => seconds - 1);
      }, 1000);
    } else if (isActive && secondsLeft === 0) {
      handleSkip();
    }
    
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [isActive, secondsLeft, handleSkip]);

  useEffect(() => {
    setSecondsLeft(durations[mode] * 60);
  }, [durations, mode]);

  useEffect(() => {
    Object.values(audioRefs).forEach(ref => {
      if (ref.current) {
        ref.current.volume = volume;
      }
    });

    if (selectedSound !== 'none' && audioRefs[selectedSound]?.current) {
      const audio = audioRefs[selectedSound].current;
      audio.loop = true;
      if (isActive) {
        audio.play().catch((error) => {
          console.warn('Audio playback failed:', error);
        });
      } else {
        audio.pause();
      }
    }
    
    return () => {
      Object.values(audioRefs).forEach(ref => {
        if (ref.current && ref.current !== audioRefs.notification.current) {
          ref.current.pause();
        }
      });
    };
  }, [isActive, selectedSound, volume, audioRefs.notification, audioRefs.rain, audioRefs.forest, audioRefs.cavern]);

  useEffect(() => {
    const onFullScreenChange = () => {
      setIsFullScreen(!!document.fullscreenElement);
    };
    if (typeof document !== 'undefined') {
      document.addEventListener('fullscreenchange', onFullScreenChange);
    }
    return () => {
      if (typeof document !== 'undefined') {
        document.removeEventListener('fullscreenchange', onFullScreenChange);
      }
    };
  }, []);
  
  const minutes = Math.floor(secondsLeft / 60);
  const seconds = secondsLeft % 60;
  const progress = (durations[mode] * 60 - secondsLeft) / (durations[mode] * 60) * 100;

  const productivityChartData = [
    { name: 'Tasks Completed', value: productivity.tasksCompleted },
    { name: 'Focus Sessions', value: productivity.focusSessions },
  ];

  if (!isMounted) {
    return null;
  }

  return (
    <TooltipProvider>
      <audio ref={audioRefs.notification} preload="metadata" src="https://assets.mixkit.co/sfx/preview/mixkit-positive-notification-951.mp3" />
      <audio ref={audioRefs.rain} preload="metadata" src="https://www.soundjay.com/nature/sounds/rain-07.mp3" />
      <audio ref={audioRefs.forest} preload="metadata" src="https://assets.mixkit.co/sfx/preview/mixkit-forest-at-night-1222.mp3" />
      <audio ref={audioRefs.cavern} preload="metadata" src="https://www.soundjay.com/nature/sounds/wind-howl-01.mp3" />
      
      <Card className="w-full max-w-md mx-auto shadow-2xl bg-card/80 backdrop-blur-sm border-white/10">
        <CardHeader className="flex-row items-center justify-between">
          <CardTitle className="text-2xl font-bold">Focus Flow</CardTitle>
          <div className="flex items-center gap-1">
            <Dialog>
              <Tooltip>
                <TooltipTrigger asChild>
                  <DialogTrigger asChild>
                    <Button variant="ghost" size="icon">
                      <BarChart2 className="w-5 h-5" />
                    </Button>
                  </DialogTrigger>
                </TooltipTrigger>
                <TooltipContent><p>Productivity</p></TooltipContent>
              </Tooltip>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Dashboard</DialogTitle>
                  <DialogDescription>
                    A summary of your productivity metrics.
                  </DialogDescription>
                </DialogHeader>
                 <div className="space-y-4">
                    <div className="text-center">
                        <p className="text-4xl font-bold">{productivity.totalFocusTime}</p>
                        <p className="text-sm text-muted-foreground">Total Focus Time (minutes)</p>
                    </div>
                    <div className="h-64">
                    <ResponsiveContainer width="100%" height="100%">
                        <BarChart data={productivityChartData}>
                        <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border) / 0.5)" />
                        <XAxis dataKey="name" stroke="hsl(var(--foreground))" />
                        <YAxis stroke="hsl(var(--foreground))" />
                        <Bar dataKey="value" fill="hsl(var(--accent))" radius={[4, 4, 0, 0]} />
                        </BarChart>
                    </ResponsiveContainer>
                    </div>
                </div>
              </DialogContent>
            </Dialog>

            <Dialog>
                <Tooltip>
                    <TooltipTrigger asChild>
                      <DialogTrigger asChild>
                        <Button variant="ghost" size="icon">
                          <Settings className="w-5 h-5" />
                        </Button>
                      </DialogTrigger>
                    </TooltipTrigger>
                    <TooltipContent><p>Settings</p></TooltipContent>
                </Tooltip>
                <DialogContent>
                    <DialogHeader>
                      <DialogTitle>Timer Settings</DialogTitle>
                       <DialogDescription>
                        Customize your focus and break intervals.
                      </DialogDescription>
                    </DialogHeader>
                    <div className="space-y-4 py-4">
                        <div>
                          <Label htmlFor="focus-duration">Focus (minutes)</Label>
                          <Input 
                            id="focus-duration" 
                            type="number" 
                            min={0}
                            value={focusDuration} 
                            onChange={(e) => setFocusDuration(Math.max(0, Number(e.target.value)))} 
                          />
                        </div>
                        <div>
                          <Label htmlFor="short-break-duration">Short Break (minutes)</Label>
                          <Input 
                            id="short-break-duration" 
                            type="number" 
                            min={0}
                            value={shortBreakDuration} 
                            onChange={(e) => setShortBreakDuration(Math.max(0, Number(e.target.value)))} 
                          />
                        </div>
                        <div>
                          <Label htmlFor="long-break-duration">Long Break (minutes)</Label>
                          <Input 
                            id="long-break-duration" 
                            type="number" 
                            min={0}
                            value={longBreakDuration} 
                            onChange={(e) => setLongBreakDuration(Math.max(0, Number(e.target.value)))} 
                          />
                        </div>
                        <div className="flex items-center space-x-2 pt-2">
                            <Switch id="auto-start-switch" checked={autoStart} onCheckedChange={setAutoStart} />
                            <Label htmlFor="auto-start-switch">Auto-start next timer</Label>
                        </div>
                    </div>
                </DialogContent>
            </Dialog>
            <Tooltip>
                <TooltipTrigger asChild>
                  <Button variant="ghost" size="icon" onClick={toggleFullScreen}>
                    {isFullScreen ? <Minimize className="w-5 h-5"/> : <Maximize className="w-5 h-5"/>}
                  </Button>
                </TooltipTrigger>
                <TooltipContent><p>Do Not Disturb</p></TooltipContent>
            </Tooltip>
          </div>
        </CardHeader>
        <CardContent className="flex flex-col items-center justify-center gap-8 text-center">
            <div className="relative w-full">
                <div className="grid grid-cols-3 gap-2">
                    <Button variant={mode === 'focus' ? 'secondary' : 'ghost'} onClick={() => handleModeChange('focus')}>Focus</Button>
                    <Button variant={mode === 'shortBreak' ? 'secondary' : 'ghost'} onClick={() => handleModeChange('shortBreak')}>Short Break</Button>
                    <Button variant={mode === 'longBreak' ? 'secondary' : 'ghost'} onClick={() => handleModeChange('longBreak')}>Long Break</Button>
                </div>
            </div>

          <div className="font-mono text-8xl md:text-9xl font-bold tracking-tighter relative">
            {minutes.toString().padStart(2, "0")}:{seconds.toString().padStart(2, "0")}
            <Progress value={progress} className="h-1 absolute -bottom-4 left-0 right-0 w-full" />
          </div>

          <div className="flex items-center gap-4">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="ghost" size="icon" className="w-12 h-12" onClick={handleReset}>
                  <RefreshCcw />
                </Button>
              </TooltipTrigger>
              <TooltipContent><p>Reset</p></TooltipContent>
            </Tooltip>
            <Button size="lg" className="w-32 h-16 text-2xl rounded-full" onClick={handleToggleActive}>
              {isActive ? <Pause className="w-8 h-8" /> : <Play className="w-8 h-8" />}
              <span className="ml-2">{isActive ? 'Pause' : 'Start'}</span>
            </Button>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="ghost" size="icon" className="w-12 h-12" onClick={handleSkip}>
                  <SkipForward />
                </Button>
              </TooltipTrigger>
              <TooltipContent><p>Skip</p></TooltipContent>
            </Tooltip>
          </div>

          <div className="w-full text-left pt-4">
            <h3 className="flex items-center gap-2 text-lg font-semibold">
              <ListChecks className="w-5 h-5"/> Tasks
            </h3>
            <form onSubmit={handleAddTask} className="flex gap-2 mt-2">
                <Input 
                  placeholder='Add a new task...' 
                  value={newTaskText} 
                  onChange={e => setNewTaskText(e.target.value)} 
                />
                <Button type="submit">Add</Button>
            </form>
            <ul className="mt-4 space-y-2 max-h-40 overflow-y-auto">
                {tasks.map(task => (
                    <li key={task.id} className="flex items-center gap-3 p-2 rounded-md bg-white/5 group hover:bg-white/10 transition-colors">
                        <Checkbox 
                          id={`task-${task.id}`} 
                          checked={task.completed} 
                          onCheckedChange={() => handleToggleTask(task.id)}
                          disabled={!!editingTaskId}
                        />
                         {editingTaskId === task.id ? (
                            <form onSubmit={handleUpdateTask} className="flex-grow flex gap-2 items-center">
                                <Input 
                                    value={editingTaskText}
                                    onChange={e => setEditingTaskText(e.target.value)}
                                    className="h-8"
                                    autoFocus
                                />
                                <Button type="submit" variant="ghost" size="icon" className="h-8 w-8 shrink-0">
                                    <CheckIcon className="w-4 h-4" />
                                </Button>
                                <Button type="button" variant="ghost" size="icon" className="h-8 w-8 shrink-0" onClick={() => setEditingTaskId(null)}>
                                    <X className="w-4 h-4" />
                                </Button>
                            </form>
                        ) : (
                          <>
                            <label 
                              htmlFor={`task-${task.id}`} 
                              className={cn("flex-grow cursor-pointer", task.completed && "line-through text-muted-foreground")}
                            >
                              {task.text}
                            </label>
                            <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                <Tooltip>
                                    <TooltipTrigger asChild>
                                        <Button variant="ghost" size="icon" className="h-7 w-7" onClick={() => handleEditTask(task)} disabled={!!editingTaskId}>
                                            <Pencil className="w-4 h-4" />
                                        </Button>
                                    </TooltipTrigger>
                                    <TooltipContent><p>Edit Task</p></TooltipContent>
                                </Tooltip>
                                <Tooltip>
                                    <TooltipTrigger asChild>
                                        <Button variant="ghost" size="icon" className="h-7 w-7" onClick={() => handleDeleteTask(task.id)} disabled={!!editingTaskId}>
                                            <Trash2 className="w-4 h-4" />
                                        </Button>
                                    </TooltipTrigger>
                                    <TooltipContent><p>Delete Task</p></TooltipContent>
                                </Tooltip>
                            </div>
                          </>
                        )}
                    </li>
                ))}
            </ul>
          </div>
        </CardContent>
        <CardFooter className="flex-col gap-4">
            <div className="text-center w-full max-w-xs">
                <p className="text-sm text-muted-foreground mb-2">Ambient Sounds</p>
                <div className="flex justify-center gap-2">
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button 
                          variant={selectedSound === 'none' ? 'secondary' : 'ghost'} 
                          size="icon" 
                          onClick={() => handleSoundChange('none')}
                        >
                          <VolumeX />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent><p>None</p></TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button 
                          variant={selectedSound === 'forest' ? 'secondary' : 'ghost'} 
                          size="icon" 
                          onClick={() => handleSoundChange('forest')}
                        >
                          <Leaf />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent><p>Forest</p></TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button 
                          variant={selectedSound === 'cavern' ? 'secondary' : 'ghost'} 
                          size="icon" 
                          onClick={() => handleSoundChange('cavern')}
                        >
                          <Wind />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent><p>Cavern</p></TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button 
                          variant={selectedSound === 'rain' ? 'secondary' : 'ghost'} 
                          size="icon" 
                          onClick={() => handleSoundChange('rain')}
                        >
                          <Droplets />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent><p>Rain</p></TooltipContent>
                    </Tooltip>
                </div>
                <div className="mt-4">
                    <Slider
                      value={[volume * 100]}
                      onValueChange={(value) => setVolume(value[0] / 100)}
                      max={100}
                      step={1}
                      disabled={selectedSound === 'none'}
                    />
                </div>
            </div>
        </CardFooter>
      </Card>
    </TooltipProvider>
  );
}
