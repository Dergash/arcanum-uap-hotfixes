; the file is for illustrative purposes only to document `patch.rs` result in-game
; starting address is assumed to be allocated at xxxxx00

push eax                        ; preserve registers before bed lookup                                              ; 1 byte
push ecx                        ;                                                                                   ; 1 byte
push edx                        ;                                                                                   ; 1 byte

cmp dword ptr ss:[esp+1C],0     ; check `is_sleeping`                                                               ; 5 bytes
je xxxxx28                      ; if not sleeping, resume original flow                                             ; 2 bytes

push dword ptr ss:[esp+2C]      ; standpoint_loc high dword (original [ESP+20])                                     ; 4 bytes
push dword ptr ss:[esp+2C]      ; standpoint_loc low dword (original [ESP+1C])                                      ; 4 bytes
call arcanum.4AD140             ; find bed on the current standpoint tile                                           ; 5 bytes
add esp,8                       ; discard int64 argument                                                            ; 3 bytes
or eax,edx                      ; OBJ_HANDLE_NULL is 0; test the complete 64-bit handle                             ; 2 bytes
test eax,eax                    ;                                                                                   ; 2 bytes
jne xxxxx28                     ; if bed exists, preserve sleeping                                                  ; 2 bytes

pop edx                         ; before waking up the NPC, restore original registers                              ; 1 byte
pop ecx                         ;                                                                                   ; 1 byte
pop eax                         ;                                                                                   ; 1 byte

jmp arcanum.4ACF6D              ; exit from patch to wake up NPC procedure                                          ; 5 bytes

pop edx                         ; xxxxx28, restoring registers before exiting to original flow                      ; 1 byte
pop ecx                         ;                                                                                   ; 1 byte
pop eax                         ;                                                                                   ; 1 byte

test ebp,200                    ; before exiting, need to call instructions from which our patch detoured from      ; 6 bytes
jmp arcanum.4ACECE              ; exit from patch to original flow when NPC wake up not needed                      ; 5 bytes
