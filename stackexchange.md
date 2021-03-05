
# How deep our experiments on non determinism are?

https://physics.stackexchange.com/questions/618717/how-deep-our-experiments-on-non-determinism-are

Disclaimer: Sorry in advance, because I've probably misunderstood something from
this introductory quantum physics lesson, or something else related to
quantum physics or math. But, hey, I need to know what it is that I'm missing.

MIT 8.04 Quantum Physics I, Spring 2013 (2013)
https://www.youtube.com/watch?v=lZ3bPUKo5zc

There's no need to watch the video unless I summarize very badly, hopefully the examples and induction will do the deal.

Letting aside how the quantum universe we live in is more concretely,
we can conceive an abstract simpler universe in which we just have
sources of particles, and two types of filters, namely Up/Down and
Left/Right and then detectors were the particles end up so we can "see
them". We can combine the filters any way we want for our experiments.

Here are some examples:

```
Depth 1:
Source--|UpDown|--Up----Detector: 50%
               |--Down--Detector: 50%

Source--|LeftRight|--Left---Detector: 50%
                  |--Right--Detector: 50%

Depth 2:
Source--|LeftRight|--Left---|UpDown|--Up----Detector: 25%
                  |                |--Down--Detector: 25%
                  |--Right--|UpDown|--Up----Detector: 25%
                                   |--Down--Detector: 25%

Depth 3:
Source--|UpDown|--Up----|UpDown|--Up----|UpDown|--Up----Detector: 50%
               |               |               |--Down--Detector: 0%
               |               |--Down--Detector: 0%
               |--Down--Detector: 50%

Source--|LeftRight|--Left---|LeftRight|--Left---|LeftRight|--Left---Detector: 50%
                  |                   |                   |--Right--Detector: 0%
                  |                   |--Right--Detector: 0%
                  |--Right--Detector: 50%

Source--|LeftRight|--Left---|UpDown|--Up----|LeftRight|--Left---Detector: 12.5%
                  |                |                  |--Right--Detector: 12.5%
                  |                |--Down--|LeftRight|--Left---Detector: 12.5%
                  |                                   |--Right--Detector: 12.5%
                  |--Right--|UpDown|--Up----|LeftRight|--Left---Detector: 12.5%
                                   |                  |--Right--Detector: 12.5%
                                   |--Down--|LeftRight|--Left---Detector: 12.5%
                                                      |--Right--Detector: 12.5%

Depth 4:
Source--|LeftRight|--Left---|UpDown|--Up----|LeftRight|--Left---|UpDown|--Up----Detector: 6.25%
                  |                |                  |                |--Down--Detector: 6.25%
                  |                |                  |--Right--|UpDown|--Up----Detector: 6.25%
                  |                |                                   |--Down--Detector: 6.25%
                  |                |--Down--|LeftRight|--Left---|UpDown|--Up----Detector: 6.25%
                  |                                   |                |--Down--Detector: 6.25%
                  |                                   |--Right--|UpDown|--Up----Detector: 6.25%
                  |                                                    |--Down--Detector: 6.25%
                  |--Right--|UpDown|--Up----|LeftRight|--Left---|UpDown|--Up----Detector: 6.25%
                                   |                  |                |--Down--Detector: 6.25%
                                   |                  |--Right--|UpDown|--Up----Detector: 6.25%
                                   |                                   |--Down--Detector: 6.25%
                                   |--Down--|LeftRight|--Left---|UpDown|--Up----Detector: 6.25%
                                                      |                |--Down--Detector: 6.25%
                                                      |--Right--|UpDown|--Up----Detector: 6.25%
                                                                       |--Down--Detector: 6.25%
```

As far as I understand, these abstractions could be implemented as
real life experiments in many ways, with electrons, protons, photons
and other particles, even molecules like carbon icosahedrons (whatever
their chemical name is) and our physics expect the same results,
because many people have probably tried with different particles and
machinery, different distances, times, angles and nobody got anything
different that could give us a clue.

Of course, I assume experimental physicists didn't stop at
depth 4. So, after so much introduction, the first question is how
deep they went? I expect a number absurdly big using mirrors and
recursion or something.

But I also want to claim some things to see if I'm getting something wrong.

CLAIM 1: For any set of experiments like these in the examples, a deterministic model can be conceived that produces the same expected results as the non deterministic one.

CLAIM 2: For any such model, a superset of experiments could be designed so that the deterministic and the deterministic models gave non equivalent results.

So how can anybody claim that our universe is intrinsically non deterministic without having done infinitely "deep" experiments on non determinism?
And also, again, how big was that number when they gave up on trying anything bigger if they ever did?
I'm pretty sure I'm not the first one that a bigger number could demonstrably restore determinism if we just knew that number and designed an experiment knowing it.

Am I claiming anything wrong? It still feels like it, but I verified for myself.
I mean, I only verified up to depth 20, but I convinced myself I can generalize to N depth.
For more context, I have a little project proving my point, presumably I'm not revealing anything that was unknown to physicists and I'm just missing something.
The project just doesn't allow N=infinite though, for I haven't figured out how to code infinite loops in rust yet.
But to reiterate, real world experiments can't do that either, so I think I'm fine on that front.

https://github.com/jtimon/qcircuits

Here's the deterministic part that feels like "cheating":

https://github.com/jtimon/qcircuits/blob/master/src/hypotheses/det_bits.rs

Sorry, I hope the questions makes sense to somebody.
If I'm missing something very obvious to some, I won't be embarrassed, I want to know what it is.
Thanks in advance for the answers.
