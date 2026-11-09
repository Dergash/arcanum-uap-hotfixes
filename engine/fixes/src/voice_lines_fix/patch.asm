; the file is for illustrative purposes only to document `patch.rs` result in-game
; starting address is assumed to be allocated at xxxxx00

test esi,esi                    ; check whether bytes remain after the 32 KB copies                         ; 2 bytes
jne xxxxx13                     ; nonzero remainder: continue with the original final read                  ; 2 bytes

pop edi                         ; restore registers saved by the original function                          ; 1 byte
pop esi                         ;                                                                           ; 1 byte
pop ebx                         ;                                                                           ; 1 byte

add esp,8000                    ; discard the 32 KB stack buffer                                            ; 6 bytes
mov eax,1                       ; return true: all file bytes were copied                                   ; 5 bytes
ret                             ;                                                                           ; 1 byte

push edi                        ; xxxxx13, replay instructions overwritten by the detour                    ; 1 byte
push 1                          ;                                                                           ; 2 bytes
lea edx,dword ptr ss:[esp+14]   ;                                                                           ; 4 bytes
jmp arcanum.5312D6              ; resume original flow at the final read                                    ; 5 bytes
