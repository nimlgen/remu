
<stdin>:	file format elf64-amdgpu

Disassembly of section .text:

0000000000001700 <E_4n132>:
	s_load_b128 s[4:7], s[0:1], null                           // 000000001700: F4080100 F8000000
	s_mov_b32 s2, s15                                          // 000000001708: BE82000F
	s_ashr_i32 s3, s15, 31                                     // 00000000170C: 86039F0F
	v_mov_b32_e32 v2, 0                                        // 000000001710: 7E040280
	s_lshl_b64 s[8:9], s[2:3], 1                               // 000000001714: 84888102
	s_load_b64 s[0:1], s[0:1], 0x10                            // 000000001718: F4040000 F8000010
	s_waitcnt lgkmcnt(0)                                       // 000000001720: BF89FC07
	s_add_u32 s6, s6, s8                                       // 000000001724: 80060806
	s_addc_u32 s7, s7, s9                                      // 000000001728: 82070907
	s_lshl_b64 s[2:3], s[2:3], 3                               // 00000000172C: 84828302
	global_load_u16 v0, v2, s[6:7]                             // 000000001730: DC4A0000 00060002
	s_add_u32 s0, s0, s2                                       // 000000001738: 80000200
	s_addc_u32 s1, s1, s3                                      // 00000000173C: 82010301
	s_load_b64 s[0:1], s[0:1], null                            // 000000001740: F4040000 F8000000
	s_waitcnt vmcnt(0)                                         // 000000001748: BF8903F7
	v_and_b32_e32 v0, 0xffff, v0                               // 00000000174C: 360000FF 0000FFFF
	s_waitcnt lgkmcnt(0)                                       // 000000001754: BF89FC07
	s_delay_alu instid0(VALU_DEP_1) | instskip(NEXT) | instid1(VALU_DEP_1)// 000000001758: BF870091
	v_add_co_u32 v0, s0, s0, v0                                // 00000000175C: D7000000 00020000
	v_add_co_ci_u32_e64 v1, null, s1, 0, s0                    // 000000001764: D5207C01 00010001
	s_add_u32 s0, s4, s2                                       // 00000000176C: 80000204
	s_addc_u32 s1, s5, s3                                      // 000000001770: 82010305
	global_store_b64 v2, v[0:1], s[0:1]                        // 000000001774: DC6E0000 00000002
	s_nop 0                                                    // 00000000177C: BF800000
	s_sendmsg sendmsg(MSG_DEALLOC_VGPRS)                       // 000000001780: BFB60003
	s_endpgm                                                   // 000000001784: BFB00000