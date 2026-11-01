// Sounds, synthesised in the browser: no files to download, and nothing whose rights are
// unclear. The art bible's rule holds: no losing sounds. A mistake gets a soft, rising "let's
// look again"; only the good moments are bright. Sound can be turned off, and the choice is kept.

export type SoundName = "tap" | "select" | "correct" | "nudge" | "combo" | "complete" | "achieved" | "live";

/** [frequency in Hz, start in s, length in s, waveform, loudness 0-1] */
type Note = [number, number, number, OscillatorType?, number?];

const C5 = 523.25, E5 = 659.25, G5 = 783.99, A5 = 880, B5 = 987.77, C6 = 1046.5, D6 = 1174.66, E6 = 1318.51, G6 = 1567.98;

const SOUNDS: Record<SoundName, Note[]> = {
  tap: [[E5, 0, 0.05, "triangle", 0.12]],
  select: [[A5, 0, 0.07, "sine", 0.16]],
  correct: [[G5, 0, 0.1, "sine", 0.22], [D6, 0.08, 0.22, "sine", 0.22]],
  nudge: [[392, 0, 0.14, "sine", 0.12], [440, 0.11, 0.18, "sine", 0.1]],
  combo: [[G5, 0, 0.08, "sine", 0.18], [B5, 0.07, 0.08, "sine", 0.18], [D6, 0.14, 0.08, "sine", 0.18], [G6, 0.21, 0.2, "sine", 0.18]],
  complete: [[C5, 0, 0.12, "triangle", 0.2], [E5, 0.1, 0.12, "triangle", 0.2], [G5, 0.2, 0.12, "triangle", 0.2], [C6, 0.3, 0.45, "triangle", 0.22], [E6, 0.3, 0.45, "sine", 0.08]],
  achieved: [
    [G5, 0, 0.1, "square", 0.06], [C6, 0.1, 0.1, "square", 0.06], [E6, 0.2, 0.1, "square", 0.06], [G6, 0.3, 0.6, "sine", 0.18],
    [C5, 0.3, 0.6, "triangle", 0.16], [E5, 0.3, 0.6, "triangle", 0.12], [G5, 0.3, 0.6, "triangle", 0.1],
  ],
  live: [[C6, 0, 0.08, "triangle", 0.18], [C6, 0.1, 0.08, "triangle", 0.18], [G6, 0.2, 0.35, "sine", 0.2]],
};

const KEY = "zidara.sound";

class Sound {
  on = $state(read());
  #ctx: AudioContext | undefined;

  toggle() {
    this.on = !this.on;
    try {
      localStorage.setItem(KEY, this.on ? "on" : "off");
    } catch {
      // Private windows may refuse storage; the choice still holds for this visit.
    }
    if (this.on) this.play("select");
  }

  /** Plays a sound now. Browsers allow audio after a tap, which is when the app calls this. */
  play(name: SoundName) {
    if (!this.on) return;
    try {
      const ctx = (this.#ctx ??= new AudioContext());
      void ctx.resume();
      const start = ctx.currentTime + 0.01;
      for (const [freq, at, length, wave = "sine", loud = 0.2] of SOUNDS[name]) {
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = wave;
        osc.frequency.value = freq;
        const t = start + at;
        gain.gain.setValueAtTime(0, t);
        gain.gain.linearRampToValueAtTime(loud, t + 0.012);
        gain.gain.exponentialRampToValueAtTime(0.0001, t + length);
        osc.connect(gain).connect(ctx.destination);
        osc.start(t);
        osc.stop(t + length + 0.02);
      }
    } catch {
      // No audio on this device: the screen still says everything the sound does.
    }
  }
}

function read(): boolean {
  try {
    return localStorage.getItem(KEY) !== "off";
  } catch {
    return true;
  }
}

export const sound = new Sound();
