
<stdin>:	file format elf64-amdgpu

Disassembly of section .text:

0000000000001700 <r_2_2_2n6>:
	s_clause 0x1                                               // 000000001700: BF850001
	s_load_b128 s[4:7], s[0:1], null                           // 000000001704: F4080100 F8000000
	s_load_b64 s[0:1], s[0:1], 0x10                            // 00000000170C: F4040000 F8000010
	s_lshl_b32 s2, s15, 1                                      // 000000001714: 8402810F
	s_delay_alu instid0(SALU_CYCLE_1) | instskip(NEXT) | instid1(SALU_CYCLE_1)// 000000001718: BF870499
	s_ashr_i32 s3, s2, 31                                      // 00000000171C: 86039F02
	s_lshl_b64 s[2:3], s[2:3], 2                               // 000000001720: 84828202
	s_waitcnt lgkmcnt(0)                                       // 000000001724: BF89FC07
	s_add_u32 s6, s6, s2                                       // 000000001728: 80060206
	s_addc_u32 s7, s7, s3                                      // 00000000172C: 82070307
	s_lshl_b32 s8, s14, 1                                      // 000000001730: 8408810E
	s_delay_alu instid0(SALU_CYCLE_1) | instskip(NEXT) | instid1(SALU_CYCLE_1)// 000000001734: BF870499
	s_ashr_i32 s9, s8, 31                                      // 000000001738: 86099F08
	s_lshl_b64 s[8:9], s[8:9], 2                               // 00000000173C: 84888208
	s_delay_alu instid0(SALU_CYCLE_1)                          // 000000001740: BF870009
	s_add_u32 s0, s0, s8                                       // 000000001744: 80000800
	s_addc_u32 s1, s1, s9                                      // 000000001748: 82010901
	s_load_b64 s[6:7], s[6:7], null                            // 00000000174C: F4040183 F8000000
	s_load_b64 s[0:1], s[0:1], null                            // 000000001754: F4040000 F8000000
	s_waitcnt lgkmcnt(0)                                       // 00000000175C: BF89FC07
	s_mul_i32 s0, s0, s6                                       // 000000001760: 96000600
	s_mul_i32 s1, s1, s7                                       // 000000001764: 96010701
	s_delay_alu instid0(SALU_CYCLE_1)                          // 000000001768: BF870009
	s_add_i32 s6, s1, s0                                       // 00000000176C: 81060001
	s_add_u32 s2, s4, s2                                       // 000000001770: 80020204
	s_addc_u32 s3, s5, s3                                      // 000000001774: 82030305
	s_ashr_i32 s15, s14, 31                                    // 000000001778: 860F9F0E
	v_dual_mov_b32 v0, 0 :: v_dual_mov_b32 v1, s6              // 00000000177C: CA100080 00000006
	s_lshl_b64 s[0:1], s[14:15], 2                             // 000000001784: 8480820E
	s_delay_alu instid0(SALU_CYCLE_1)                          // 000000001788: BF870009
	s_add_u32 s0, s2, s0                                       // 00000000178C: 80000002
	s_addc_u32 s1, s3, s1                                      // 000000001790: 82010103
	global_store_b32 v0, v1, s[0:1]                            // 000000001794: DC6A0000 00000100
	s_nop 0                                                    // 00000000179C: BF800000
	s_sendmsg sendmsg(MSG_DEALLOC_VGPRS)                       // 0000000017A0: BFB60003
	s_endpgm                                                   // 0000000017A4: BFB00000