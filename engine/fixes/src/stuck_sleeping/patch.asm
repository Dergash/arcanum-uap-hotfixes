; the file is for illustative purposes only to document `patch.rs` result on how the assembly of the patch should look in-game
; starting address is assumed to be allocated at xxxxx00

push eax                        ; 'ai_is_day' modifies registers, store them before calling it                      ; 1 byte
push ecx                        ;                                                                                   ; 1 byte
push edx                        ;                                                                                   ; 1 byte

call arcanum.4ACD90             ; call `ai_is_day` which will set 1 to EAX if between 6 and 21 hours                ; 5 byte
cmp eax,1                       ;                                                                                   ; 3 bytes
jne xxxxx1C                     ; if it's night, no cause to wake NPC, path to original flow                        ; 2 bytes

cmp dword ptr ss:[esp+1C],0     ; if it's day, need to check `is_sleeping` flag on NPC                              ; 5 bytes
je xxxxx1C                      ; if NPC not sleeping, no cause to wake, path to original flow                      ; 2 bytes

pop edx                         ; before waking up the NPC, restore original registers                              ; 1 byte
pop ecx                         ;                                                                                   ; 1 byte
pop eax                         ;                                                                                   ; 1 byte

jmp arcanum.4ACF6D              ; exit from patch to wake up NPC procedure                                          ; 5 bytes

pop edx                         ; xxxxx1C, restoring registers before exiting to original flow                      ; 1 byte
pop ecx                         ;                                                                                   ; 1 byte
pop eac                         ;                                                                                   ; 1 byte

test ebp,200                    ; before exiting, need to call instructions from which our patch detoured from      ; 6 bytes
jmp arcanum.4ACECE              ; exit from patch to original flow when NPC wake up not needed                      ; 5 bytes
