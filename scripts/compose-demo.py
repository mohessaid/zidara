#!/usr/bin/env python3
"""Composes the recorded demo (web/tests/demo) into one 1080p film: the learner's phone and the
teacher's studio side by side, the studio placed where it opened, English captions per step.
Output: pitch/demo/zidara-demo.mp4.
Run after: cd web && npx playwright test --config playwright.demo.config.ts"""
import json, subprocess, tempfile
from pathlib import Path

root = Path(__file__).resolve().parent.parent
timeline = json.loads((root / "web/test-results/demo/timeline.json").read_text())
at = timeline["teacherAt"]
font = "/usr/share/fonts/truetype/noto/NotoSans-Bold.ttf"
out = root / "pitch/demo/zidara-demo.mp4"
out.parent.mkdir(parents=True, exist_ok=True)

# Each step of the recording marked its own time and caption (web/tests/demo/demo.spec.ts).
marks = timeline["marks"]
captions = [(m["t"], (marks[k + 1]["t"] if k + 1 < len(marks) else timeline["end"]), m["caption"]) for k, m in enumerate(marks)]
labels = [("Learner phone · offline-first", 470), ("Teacher studio", 1220)]

with tempfile.TemporaryDirectory() as texts:
    def textfile(i: int, text: str) -> str:
        # From a file, so quotes and colons in the text need no escaping.
        path = Path(texts) / f"{i}.txt"
        path.write_text(text)
        return str(path)

    draws = [f"drawtext=fontfile={font}:textfile={textfile(i, t)}:fontsize=34:fontcolor=0x5B6475:x={x}:y=40"
             for i, (t, x) in enumerate(labels)]
    draws += [f"drawtext=fontfile={font}:textfile={textfile(10 + i, t)}:fontsize=40:fontcolor=0x1F2A44:"
              f"x=(w-text_w)/2:y=990:enable='between(t,{a},{b})'" for i, (a, b, t) in enumerate(captions)]
    graph = ";".join([
        "color=c=0xFFF8EC:s=1920x1080:r=25[bg]",
        "[0:v]fps=25,scale=-2:880,setsar=1[l]",
        f"[1:v]fps=25,scale=-2:880,setsar=1,tpad=start_duration={at}:start_mode=add:color=0xFFF8EC[r]",
        "[bg][l]overlay=x=430:y=95:shortest=1[a]",
        "[a][r]overlay=x=1100:y=95:eof_action=pass[b]",
        "[b]" + ",".join(draws) + "[out]",
    ])
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", timeline["learner"], "-i", timeline["teacher"],
                    "-filter_complex", graph, "-map", "[out]", "-c:v", "libx264", "-preset", "slow", "-crf", "22",
                    "-pix_fmt", "yuv420p", "-movflags", "+faststart", str(out)], check=True)
print(subprocess.run(["ffprobe", "-v", "error", "-show_entries", "format=duration,size", "-of", "csv=p=0", str(out)],
                     capture_output=True, text=True).stdout.strip())
