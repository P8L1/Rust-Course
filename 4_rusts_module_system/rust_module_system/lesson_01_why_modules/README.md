# Lesson 01: Why we do not keep everything in one file

//~ Up to this point we have always been writing all of our code in one file which lives in the default module.
//~ For any project this will not suffice. Imagine how long your IDE would take to load a file with 100K+ lines of code. Imagine how hard it would be to find specific code within your project.
//~ Imagine the security risks it would impose.

//~ To solve these problems in the real world we split projects across lots of files. My personal rule of thumb is to keep all your files at under 200 lines of code where possible. Also split files by feature.

//~ Lets take a look at another problem; you are in charge of writing an authentication service for a backend. You don't want any other parts of the codebase to have access to the source code.
//~ If other parts of the codebase have access to your authentication code then it would be easy for other employees at the company to leak how authentication at the company works. This poses a security risk.

//~ So we also sometimes want to write code that other parts of the project can use but can't necessarily read. This is called encapsulation.
