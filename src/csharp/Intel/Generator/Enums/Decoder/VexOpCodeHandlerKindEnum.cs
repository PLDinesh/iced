/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

using System.Linq;

namespace Generator.Enums.Decoder {
	static class VexOpCodeHandlerKindEnum {
		const string? documentation = null;

		internal enum Enum : byte {
			Invalid,
			Invalid2,
			Dup,
			Invalid_NoModRM,
			Bitness_DontReadModRM,
			HandlerReference,
			ArrayReference,
			RM,
			Group,
			W,
			MandatoryPrefix2_1,
			MandatoryPrefix2_4,
			MandatoryPrefix2_NoModRM,
			VectorLength_NoModRM,
			VectorLength,
			Ed_V_Ib,
			Ev_VX,
			G_VK,
			Gv_Ev_Gv,
			Gv_Ev_Ib,
			Gv_Ev_Id,
			Gv_GPR_Ib,
			Gv_Gv_Ev,
			Gv_RX,
			Gv_W,
			GvM_VX_Ib,
			HRIb,
			Hv_Ed_Id,
			Hv_Ev,
			M,
			MHV,
			M_VK,
			MV,
			rDI_VX_RX,
			RdRq,
			Simple,
			VHEv,
			VHEvIb,
			VHIs4W,
			VHIs5W,
			VHM,
			VHW_2,
			VHW_3,
			VHW_4,
			VHWIb_2,
			VHWIb_4,
			VHWIs4,
			VHWIs5,
			VK_HK_RK,
			VK_R,
			VK_RK,
			VK_RK_Ib,
			VK_WK,
			VM,
			VW_2,
			VW_3,
			VWH,
			VWIb_2,
			VWIb_3,
			VX_Ev,
			VX_VSIB_HX,
			WHV,
			WV,
			WVIb,
		}

		static EnumValue[] GetValues() =>
			typeof(Enum).GetFields().Where(a => a.IsLiteral).Select(a => new EnumValue((uint)(Enum)a.GetValue(null)!, a.Name, CommentAttribute.GetDocumentation(a))).ToArray();

		public static readonly EnumType Instance = new EnumType(TypeIds.VexOpCodeHandlerKind, documentation, GetValues(), EnumTypeFlags.None);
	}
}
