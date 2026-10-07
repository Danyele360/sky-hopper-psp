from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
root=Path(__file__).resolve().parents[1]
captures=root/'docs/captures'
frames=[]
for path in sorted(captures.glob('*.rgba')):
    image=Image.frombytes('RGBA',(480,272),path.read_bytes())
    if path.stem.startswith('demo-'):frames.append(image.convert('RGB').resize((960,544),Image.Resampling.NEAREST))
    else:image.save(captures/f'{path.stem}.png')
if frames:frames[0].save(root/'assets/animations/gameplay.gif',save_all=True,append_images=frames[1:],duration=133,loop=0)
menu=Image.open(captures/'menu.png').convert('RGB')
menu.save(root/'assets/runtime/pic1.png')
icon=Image.open(root/'assets/runtime/sky.png').convert('RGB').resize((144,80),Image.Resampling.NEAREST)
robot=Image.open(root/'assets/runtime/robot.png').crop((0,96,48,144)).resize((68,68),Image.Resampling.NEAREST)
icon.paste(robot,(76,8),robot)
draw=ImageDraw.Draw(icon)
def title_font(size):
    # Reuse a local font if present; no Windows font files are redistributed.
    for name in ('C:/Windows/Fonts/consolab.ttf', 'DejaVuSansMono-Bold.ttf'):
        try:
            return ImageFont.truetype(name, size)
        except OSError:
            pass
    return ImageFont.load_default(size=size)
draw.text((6,17),'SKY',fill='#effaff',font=title_font(20))
draw.text((6,41),'HOPPER',fill='#82e4ff',font=title_font(14))
icon.save(root/'assets/runtime/icon0.png')
print('Exported screenshots, gameplay GIF and XMB icon/background')
