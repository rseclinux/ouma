# The policy AI policy
This file describes usage of AI and LLM-derived code.

## Don't
We are highly against the usage of code in the libc that has been blatantly "copy-pasted" from AI/LLM chat bots without reviewing it, refractoring and properly testing it. ALL AI comments without proper attribution to chat bots and/or other types of LLM/AI WILL be denied and if someone will keep on pushing such pull requests, patches then this person WILL be banned from project.

## Do
You may however, prototype using AI chat bots/LLMs, first you need to define what are you going to prototype, ask chat bot to "sketch" the supposed implementation AND THEN YOU MUST review it, test it and you MUST NOT copy paste produced output into patch, pull requests. Whether [I](https://codeberg.org/gnu2) ask you about the implementation specifics, whether it be global variables, small routines or a full implementation and if you won't be able to give a concrete answer then your patch WILL be denied, so new contributors MAY ask the original commiter a question regarding specifics of the implementation and get proper understanding. AI can help with catching small bugs or code inconsistencies, anything related to pattern matching and recognition, that is GOOD usage of AI, letting it do the whole implementation is BAD usage of AI.
Remember, YOU ARE 100% RESPONSIBLE FOR WHAT ARE YOU SUBMITTING, REVIEWING, TESTING and a deterministic program such as chat bot
is NOT responsible on itself and the person submitting AI code WILL answer the question if AI-derived code is broken.
