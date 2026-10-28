; the file is for illustative purposes only to document `patch.rs` result on how the assembly of the patch should look in-game
; starting address is assumed to be allocated at xxxxx00

push eax                        ; 'ai_is_day' modifies registers, store them before calling it
push ecx                        ;
push edx                        ;

call arcanum.4ACD90             ; call `ai_is_day` which will set 1 to EAX if between 6 and 21 hours 
cmp eax,1                       ;
jne xxxxx1C                     ; if it's night, no cause to wake NPC, path to original flow

cmp dword ptr ss:[esp+1C],0     ; if it's day, need to check `is_sleeping` flag on NPC
je xxxxx1C                      ; if NPC not sleeping, no cause to wake, path to original flow

pop edx                         ; before waking up the NPC, restore original registers
pop ecx                         ;
pop eax                         

jmp arcanum.4ACF6D              ; exit from patch to wake up NPC procedure

pop edx                         ; xxxxx1C, restoring registers before exiting to original flow 
pop ecx                         ;
pop eac                         ;

test ebp,200                    ; before exiting, need to call instructions from which our patch detoured from
jmp arcanum.4ACECE              ; exit from patch to original flow when NPC wake up not needed
