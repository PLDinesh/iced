// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

namespace Generator {
	static class TypeIds {
		public static readonly TypeId OrigCodeValues = new(nameof(OrigCodeValues));
		public static readonly TypeId RemovedCodeValues = new(nameof(RemovedCodeValues));
		public static readonly TypeId InstructionDefs = new(nameof(InstructionDefs));
		public static readonly TypeId EncoderTypes = new(nameof(EncoderTypes));
		public static readonly TypeId InstrInfoTypes = new(nameof(InstrInfoTypes));
		public static readonly TypeId GasCtorInfos = new(nameof(GasCtorInfos));
		public static readonly TypeId IntelCtorInfos = new(nameof(IntelCtorInfos));
		public static readonly TypeId MasmCtorInfos = new(nameof(MasmCtorInfos));
		public static readonly TypeId NasmCtorInfos = new(nameof(NasmCtorInfos));
		public static readonly TypeId FastFmtTblInfos = new(nameof(FastFmtTblInfos));
		public static readonly TypeId MemorySizeDefs = new(nameof(MemorySizeDefs));
		public static readonly TypeId RegisterDefs = new(nameof(RegisterDefs));
		public static readonly TypeId DecoderTables = new(nameof(DecoderTables));
		public static readonly TypeId TupleTypeTable = new(nameof(TupleTypeTable));
		public static readonly TypeId OpCodeOperandKindDefs = new(nameof(OpCodeOperandKindDefs));

		public static readonly TypeId IcedConstants = new(nameof(IcedConstants));
		public static readonly TypeId DecoderTestParserConstants = new(nameof(DecoderTestParserConstants));
		public static readonly TypeId DecoderConstants = new(nameof(DecoderConstants));
		public static readonly TypeId ExportedPythonTypes = new(nameof(ExportedPythonTypes));

		public static readonly TypeId Code = new(nameof(Code));
		public static readonly TypeId CodeSize = new(nameof(CodeSize));
		public static readonly TypeId CpuidFeature = new(nameof(CpuidFeature));
		public static readonly TypeId CpuidFeatureInternal = new(nameof(CpuidFeatureInternal));
		public static readonly TypeId DecoderError = new(nameof(DecoderError));
		public static readonly TypeId DecoderOptions = new(nameof(DecoderOptions));
		public static readonly TypeId DecoderTestOptions = new(nameof(DecoderTestOptions));
		public static readonly TypeId EvexOpCodeHandlerKind = new(nameof(EvexOpCodeHandlerKind));
		public static readonly TypeId MvexOpCodeHandlerKind = new(nameof(MvexOpCodeHandlerKind));
		public static readonly TypeId HandlerFlags = new(nameof(HandlerFlags));
		public static readonly TypeId LegacyHandlerFlags = new(nameof(LegacyHandlerFlags));
		public static readonly TypeId MemorySize = new(nameof(MemorySize));
		public static readonly TypeId BroadcastToKind = new(nameof(BroadcastToKind));
		public static readonly TypeId FastMemoryKeywords = new(nameof(FastMemoryKeywords));
		public static readonly TypeId IntelMemoryKeywords = new(nameof(IntelMemoryKeywords));
		public static readonly TypeId MasmMemoryKeywords = new(nameof(MasmMemoryKeywords));
		public static readonly TypeId NasmMemoryKeywords = new(nameof(NasmMemoryKeywords));
		public static readonly TypeId LegacyOpCodeHandlerKind = new(nameof(LegacyOpCodeHandlerKind));
		public static readonly TypeId PseudoOpsKind = new(nameof(PseudoOpsKind));
		public static readonly TypeId Register = new(nameof(Register));
		public static readonly TypeId RegisterKind = new(nameof(RegisterKind));
		public static readonly TypeId RegisterClass = new(nameof(RegisterClass));
		public static readonly TypeId SerializedDataKind = new(nameof(SerializedDataKind));
		public static readonly TypeId TupleType = new(nameof(TupleType));
		public static readonly TypeId VexOpCodeHandlerKind = new(nameof(VexOpCodeHandlerKind));
		public static readonly TypeId Mnemonic = new(nameof(Mnemonic));
		public static readonly TypeId GasCtorKind = new(nameof(GasCtorKind));
		public static readonly TypeId IntelCtorKind = new(nameof(IntelCtorKind));
		public static readonly TypeId MasmCtorKind = new(nameof(MasmCtorKind));
		public static readonly TypeId NasmCtorKind = new(nameof(NasmCtorKind));
		public static readonly TypeId GasSizeOverride = new(nameof(GasSizeOverride));
		public static readonly TypeId GasInstrOpInfoFlags = new(nameof(GasInstrOpInfoFlags));
		public static readonly TypeId IntelSizeOverride = new(nameof(IntelSizeOverride));
		public static readonly TypeId IntelBranchSizeInfo = new(nameof(IntelBranchSizeInfo));
		public static readonly TypeId IntelInstrOpInfoFlags = new(nameof(IntelInstrOpInfoFlags));
		public static readonly TypeId MasmInstrOpInfoFlags = new(nameof(MasmInstrOpInfoFlags));
		public static readonly TypeId NasmSignExtendInfo = new(nameof(NasmSignExtendInfo));
		public static readonly TypeId NasmSizeOverride = new(nameof(NasmSizeOverride));
		public static readonly TypeId NasmBranchSizeInfo = new(nameof(NasmBranchSizeInfo));
		public static readonly TypeId NasmInstrOpInfoFlags = new(nameof(NasmInstrOpInfoFlags));
		public static readonly TypeId NasmMemorySizeInfo = new(nameof(NasmMemorySizeInfo));
		public static readonly TypeId NasmFarMemorySizeInfo = new(nameof(NasmFarMemorySizeInfo));
		public static readonly TypeId FastFmtFlags = new(nameof(FastFmtFlags));
		public static readonly TypeId RoundingControl = new(nameof(RoundingControl));
		public static readonly TypeId OpKind = new(nameof(OpKind));
		public static readonly TypeId InstrScale = new(nameof(InstrScale));
		public static readonly TypeId InstrFlags1 = new(nameof(InstrFlags1));
		public static readonly TypeId MvexInstrFlags = new(nameof(MvexInstrFlags));
		public static readonly TypeId VectorLength = new(nameof(VectorLength));
		public static readonly TypeId MandatoryPrefixByte = new(nameof(MandatoryPrefixByte));
		public static readonly TypeId StateFlags = new(nameof(StateFlags));
		public static readonly TypeId EncodingKind = new(nameof(EncodingKind));
		public static readonly TypeId FlowControl = new(nameof(FlowControl));
		public static readonly TypeId OpCodeOperandKind = new(nameof(OpCodeOperandKind));
		public static readonly TypeId RflagsBits = new(nameof(RflagsBits));
		public static readonly TypeId ImpliedAccess = new(nameof(ImpliedAccess));
		public static readonly TypeId RflagsInfo = new(nameof(RflagsInfo));
		public static readonly TypeId OpInfo0 = new(nameof(OpInfo0));
		public static readonly TypeId OpInfo1 = new(nameof(OpInfo1));
		public static readonly TypeId OpInfo2 = new(nameof(OpInfo2));
		public static readonly TypeId OpInfo3 = new(nameof(OpInfo3));
		public static readonly TypeId OpInfo4 = new(nameof(OpInfo4));
		public static readonly TypeId InfoFlags1 = new(nameof(InfoFlags1));
		public static readonly TypeId InfoFlags2 = new(nameof(InfoFlags2));
		public static readonly TypeId InstrInfoConstants = new(nameof(InstrInfoConstants));
		public static readonly TypeId OpAccess = new(nameof(OpAccess));
		public static readonly TypeId ConditionCode = new(nameof(ConditionCode));
		public static readonly TypeId MiscInstrInfoTestConstants = new(nameof(MiscInstrInfoTestConstants));
		public static readonly TypeId InstructionInfoKeys = new(nameof(InstructionInfoKeys));
		public static readonly TypeId RflagsBitsConstants = new(nameof(RflagsBitsConstants));
		public static readonly TypeId MemorySizeFlags = new(nameof(MemorySizeFlags));
		public static readonly TypeId RegisterFlags = new(nameof(RegisterFlags));
		public static readonly TypeId MiscSectionNames = new(nameof(MiscSectionNames));
		public static readonly TypeId LegacyOpCodeTable = new(nameof(LegacyOpCodeTable));
		public static readonly TypeId VexOpCodeTable = new(nameof(VexOpCodeTable));
		public static readonly TypeId XopOpCodeTable = new(nameof(XopOpCodeTable));
		public static readonly TypeId EvexOpCodeTable = new(nameof(EvexOpCodeTable));
		public static readonly TypeId MvexOpCodeTable = new(nameof(MvexOpCodeTable));
		public static readonly TypeId LegacyOpKind = new(nameof(LegacyOpKind));
		public static readonly TypeId VexOpKind = new(nameof(VexOpKind));
		public static readonly TypeId XopOpKind = new(nameof(XopOpKind));
		public static readonly TypeId EvexOpKind = new(nameof(EvexOpKind));
		public static readonly TypeId MvexOpKind = new(nameof(MvexOpKind));
		public static readonly TypeId MandatoryPrefix = new(nameof(MandatoryPrefix));
		public static readonly TypeId OpCodeTableKind = new(nameof(OpCodeTableKind));
		public static readonly TypeId DisplSize = new(nameof(DisplSize));
		public static readonly TypeId ImmSize = new(nameof(ImmSize));
		public static readonly TypeId EncoderFlags = new(nameof(EncoderFlags));
		public static readonly TypeId EncFlags1 = new(nameof(EncFlags1));
		public static readonly TypeId EncFlags2 = new(nameof(EncFlags2));
		public static readonly TypeId EncFlags3 = new(nameof(EncFlags3));
		public static readonly TypeId OpCodeInfoFlags1 = new(nameof(OpCodeInfoFlags1));
		public static readonly TypeId OpCodeInfoFlags2 = new(nameof(OpCodeInfoFlags2));
		public static readonly TypeId DecOptionValue = new(nameof(DecOptionValue));
		public static readonly TypeId InstrStrFmtOption = new(nameof(InstrStrFmtOption));
		public static readonly TypeId WBit = new(nameof(WBit));
		public static readonly TypeId LBit = new(nameof(LBit));
		public static readonly TypeId OpCodeInfoKeys = new(nameof(OpCodeInfoKeys));
		public static readonly TypeId OpCodeInfoFlags = new(nameof(OpCodeInfoFlags));
		public static readonly TypeId LKind = new(nameof(LKind));
		public static readonly TypeId RepPrefixKind = new(nameof(RepPrefixKind));
		public static readonly TypeId OpSize = new(nameof(OpSize));
		public static readonly TypeId RelocKind = new(nameof(RelocKind));
		public static readonly TypeId BlockEncoderOptions = new(nameof(BlockEncoderOptions));
		public static readonly TypeId NumberBase = new(nameof(NumberBase));
		public static readonly TypeId MemorySizeOptions = new(nameof(MemorySizeOptions));
		public static readonly TypeId FormatMnemonicOptions = new(nameof(FormatMnemonicOptions));
		public static readonly TypeId PrefixKind = new(nameof(PrefixKind));
		public static readonly TypeId DecoratorKind = new(nameof(DecoratorKind));
		public static readonly TypeId NumberKind = new(nameof(NumberKind));
		public static readonly TypeId FormatterTextKind = new(nameof(FormatterTextKind));
		public static readonly TypeId SymbolFlags = new(nameof(SymbolFlags));
		public static readonly TypeId GasInstrOpKind = new(nameof(GasInstrOpKind));
		public static readonly TypeId IntelInstrOpKind = new(nameof(IntelInstrOpKind));
		public static readonly TypeId MasmInstrOpKind = new(nameof(MasmInstrOpKind));
		public static readonly TypeId NasmInstrOpKind = new(nameof(NasmInstrOpKind));
		public static readonly TypeId FormatterFlowControl = new(nameof(FormatterFlowControl));
		public static readonly TypeId OptionsProps = new(nameof(OptionsProps));
		public static readonly TypeId MasmSymbolTestFlags = new(nameof(MasmSymbolTestFlags));
		public static readonly TypeId FormatterSyntax = new(nameof(FormatterSyntax));
		public static readonly TypeId CC_b = new(nameof(CC_b));
		public static readonly TypeId CC_ae = new(nameof(CC_ae));
		public static readonly TypeId CC_e = new(nameof(CC_e));
		public static readonly TypeId CC_ne = new(nameof(CC_ne));
		public static readonly TypeId CC_be = new(nameof(CC_be));
		public static readonly TypeId CC_a = new(nameof(CC_a));
		public static readonly TypeId CC_p = new(nameof(CC_p));
		public static readonly TypeId CC_np = new(nameof(CC_np));
		public static readonly TypeId CC_l = new(nameof(CC_l));
		public static readonly TypeId CC_ge = new(nameof(CC_ge));
		public static readonly TypeId CC_le = new(nameof(CC_le));
		public static readonly TypeId CC_g = new(nameof(CC_g));
		public static readonly TypeId CodeAsmMemoryOperandSize = new(nameof(CodeAsmMemoryOperandSize));
		public static readonly TypeId TestInstrFlags = new(nameof(TestInstrFlags));
		public static readonly TypeId MvexConvFn = new(nameof(MvexConvFn));
		public static readonly TypeId MvexEHBit = new(nameof(MvexEHBit));
		public static readonly TypeId NonDestructiveOpKind = new(nameof(NonDestructiveOpKind));
		public static readonly TypeId MvexInfoFlags = new(nameof(MvexInfoFlags));
		public static readonly TypeId MvexRegMemConv = new(nameof(MvexRegMemConv));
		public static readonly TypeId MvexTupleTypeLutKind = new(nameof(MvexTupleTypeLutKind));
	}
}
