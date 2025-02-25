# wod

Create a CLI app. Use for reference https://github.com/astral-sh/ruff/tree/main or https://github.com/astral-sh/ruff/tree/main.

The idea is to have a way of creating wods that can be written to a file (both specific format and JSON initially), to be then served in a blog (Hugo blog post this program should work wherever the repo for the blog is).
The blogs have to be defined such as we can create a WOD from the CLI (that will be written to a file in a given directory (*here* as default)). The workouts can be updated. Each WOD will be represented by a single file with the date. A wod can have N workouts. Each workout can have N movements, plus a style. All the allowed movements have to be predefined, to avoid confusing names. Different format styles (EMOM, AMRAP...), and rep schemes (1x4 @ 80%) using a given parser. Define this in terms of a grammar??
- [ ] Create the movements 


Example:

```shell
# Create new WOD (which will create a file)
wod <name of the wod, if not informed, will be the current day>
# Add a workout to the file
# A workout will be a vec of movements
wod add <block>
# A block is an array of movements and rep schemes (THIS HAS TO BE CHANGED IN movement.rs)
# One can add as many blocks as needed
wod add <block 1>
wod add <block 2>
# OR pass them all at once (how to do this with the command line?)
```

### Create a new wod

```shell
wod  # by default will create the folder in the current directory
# or specify the date
wod 18-02-2024
```


### For the blog

Use Hugo with: https://blowfish.page/samples/
https://github.com/nunocoracao/blowfish

deploy to netlify: https://www.netlify.com/blog/2016/10/27/a-step-by-step-guide-deploying-a-static-site-or-single-page-app/

## TODO LIST

- [ ] Write something to the file (initially the same content provided)
- [ ] Check if exists, and then append to the file
- [ ] Parse blocks/workouts to the corresponding structs
    Should be written at the end (and properly as markdown)

---
Example commands:

```shell
# For time
wod add "ft 21-15-9 thrusters,pull ups"
# Rounds
wod add "5rd 20 double under, 30cal row"
# AMRAP
wod add "amrap-12m 10 db snatch, 1 ring muscle up"
# EMOM
wod add "emom-90s(-alt) 21-15-9 thrusters,pull ups"
# For an emom, it's written as the name
```

All start with the WorkoutType (for time, rounds, amrap, emom)
- For EMOMs, can be alternating, or always the same (start with this one)

---

I may need to write a lexer to parse such strings?
see differences between grammar, lexer and such, and write down first the different possibilities.

- Type of workout
FT: For Time, can be minus
AMRAP: ...
EMOM: ...
- Repetitions
REP (number of repetitions), but this can be associated to meters, or calories, or seconds...?
- Movements
Then comma separated movements
- Add names to the wod (with ""), if not informed, let it blank
---
FT REP-REP-REP... MOV, MOV
---

### Asked to ChatGPT

- I want a way of sharing a list of workouts with some friends. I was thinking of building a simple static web using Hugo. I would just need a simple tool to render the blog posts, that will be the workouts, associated with the given date. Is Hugo the best approach? in case it is, what would be the most suitable theme? Can I have the posts nested by dates? meaning, a year can be clicked as a dropdown and then the months

And to the answer

- that would be perfect. I will go with Blowfish, and would like a free option to deploy the blog. Maybe using netlify??