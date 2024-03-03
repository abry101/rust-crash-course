#

## Git Config

- ``` git config -l ``` or ``` git config --list ```
- ``` git config --global user.name "John" ```
- ``` git config --global user.email "signups@fabiopacifici.com" ```
- ``` git config --global credential.helper cache ```

## Initialize a Git repo

1. ``` git init ``` :- this will implicitly name the branch **"master"**  
2. ``` git init -b branch_name ``` or ``` git init --initial-branch=branch_name ```:- we can avoid/override the above by explicitly specifying the branch with **-b** flag
3.
    ```
    > git init && git checkout -b main 
    OR
    > git init
    > git checkout -b main dir
    
    ```

## Git Branch

- ``` git branch ``` :- list branch
- ``` git branch branch_name ``` :- create branch
- ``` git branch -d branch_name ``` :- delete branch
- ``` git checkout branch_name ``` :- switch branch
- ``` git merge branch_name ``` :- merge current branch with *"branch_name"*
- ``` git checkout -b branch_name ``` :- create and switch branch immediately
