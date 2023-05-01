// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2019-2022 Second State INC

#include "plugin/plugin.h"
#include "po/helper.h"
#include "runtime/callingframe.h"
#include "runtime/instance/module.h"
#include <algorithm>

#include <tesseract/baseapi.h>
#include <leptonica/allheaders.h>

namespace {

using namespace std::literals::string_view_literals;
using namespace WasmEdge;

// PO::Option<std::string> StringOpt(PO::Description("string to return"sv),
//                                   PO::MetaVar("STRING"sv),
//                                   PO::DefaultValue<std::string>("hello"));

// PO::Option<PO::Toggle> UpperOpt(PO::Description("return in upper case"sv));

// void addOptions(PO::ArgumentParser &Parser) noexcept {
//   Parser.add_option("string"sv, StringOpt).add_option("upper"sv, UpperOpt);
// }

class ReturnObtainedString : public Runtime::HostFunction<ReturnObtainedString> {
public:
  ReturnObtainedString() {}
  Expect<void> body(const Runtime::CallingFrame &Frame,uint32_t RBufPtr,
                    uint32_t RBufLen, uint32_t WBufPtr,
                    uint32_t WBufLen) {
    // Check memory instance from module.
    auto *MemInst = Frame.getMemoryByIndex(0);
    if (MemInst == nullptr) {
      return Unexpect(ErrCode::Value::HostFuncError);
    }
    // Validate range of the buffer to read from.
    char *const RBuf = MemInst->getPointer<char *>(RBufPtr, RBufLen);
    if (unlikely(RBuf == nullptr)) {
      return Unexpect(ErrCode::Value::HostFuncError);
    }

    // Validate range of the buffer to write to.
    char *const WBuf = MemInst->getPointer<char *>(WBufPtr, WBufLen);
    if (unlikely(WBuf == nullptr)) {
      return Unexpect(ErrCode::Value::HostFuncError);
    }

    // What was done in getstring.cpp example. // instead my method
    // char *const End = std::copy(String.begin(), String.end(), Buf);
    // // *Written = End - Buf;
    tesseract::TessBaseAPI *api = new tesseract::TessBaseAPI();
    // Initialize tesseract-ocr with English, without specifying tessdata path
    if (api->Init(NULL, "eng")) {
        fprintf(stderr, "Could not initialize tesseract.\n");
        exit(1);
    }
    Pix *image = pixRead(RBuf);
    api->SetImage(image);
    api->Recognize(0);
    
    tesseract::PageIteratorLevel level = tesseract::RIL_WORD;
    const char *outText = api->GetTSVText(level);
    // instead my method
    std::strcpy(WBuf,outText);    
    api->End();
  //   delete api;
    delete [] outText; // USE WHEN USING TESS API
    pixDestroy(&image);
    return {};
  }

private:
  std::string_view String;
};

class PluginModule : public Runtime::Instance::ModuleInstance {
public:
  PluginModule() : ModuleInstance("wasi_string_test") {
    addHostFunc("return_obtained_string", std::make_unique<ReturnObtainedString>());
  }
};

Runtime::Instance::ModuleInstance *
create(const Plugin::PluginModule::ModuleDescriptor *) noexcept {
  return new PluginModule;
}

Plugin::Plugin::PluginDescriptor Descriptor{
    .Name = "wasi_string_test",
    .Description = "Plugin to test behaviour of strings",
    .APIVersion = Plugin::Plugin::CurrentAPIVersion,
    .Version = {0, 10, 0, 0},
    .ModuleCount = 1,
    .ModuleDescriptions =
        (Plugin::PluginModule::ModuleDescriptor[]){
            {
                .Name = "wasi_string_test",
                .Description = "",
                .Create = create,
            },
        },
    .AddOptions = nullptr,
};

Plugin::PluginRegister Register(&Descriptor);

} // namespace
