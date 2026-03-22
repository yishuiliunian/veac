# Video Editing as Code: 综合调研报告

> 调研日期: 2026-03-21
> 范围: DSL设计、开源项目、商业产品、学术研究、AI Agent适配性

---

## 目录

1. [概述与动机](#1-概述与动机)
2. [现有项目和工具](#2-现有项目和工具)
   - 2.1 [专用视频编辑DSL项目](#21-专用视频编辑dsl项目)
   - 2.2 [代码驱动的视频生成框架](#22-代码驱动的视频生成框架)
   - 2.3 [商业API服务 (JSON-to-Video)](#23-商业api服务-json-to-video)
   - 2.4 [传统NLE脚本系统](#24-传统nle脚本系统)
3. [相关技术生态](#3-相关技术生态)
   - 3.1 [交换格式与标准](#31-交换格式与标准)
   - 3.2 [多媒体框架](#32-多媒体框架)
   - 3.3 [帧服务器脚本系统](#33-帧服务器脚本系统)
   - 3.4 [节点式合成脚本](#34-节点式合成脚本)
4. [学术研究](#4-学术研究)
5. [现有DSL/Schema设计对比分析](#5-现有dslschema设计对比分析)
6. [语言设计关键考量](#6-语言设计关键考量)
7. [AI Agent适配性考量](#7-ai-agent适配性考量)
8. [结论与建议](#8-结论与建议)

---

## 1. 概述与动机

"Video Editing as Code" (VEAC) 是一种将视频编辑过程以代码或声明式描述来表达的范式。其核心思想是：视频编辑不再依赖图形界面的鼠标拖拽操作，而是通过一种领域特定语言 (DSL) 来精确描述视频的组合、剪辑、特效和输出。

这个方向在2024-2026年间因以下原因获得了显著关注：

- **AI Agent的崛起**: LLM需要结构化的中间表示来操控视频编辑，而非生成脆弱的命令行代码
- **视频自动化需求**: 大规模视频生产（个性化视频、数据驱动视频）需要程序化方法
- **可复现性**: 代码描述的编辑过程天然可版本控制、可复现
- **流水线集成**: 需要将视频编辑嵌入CI/CD、API服务、批处理流程

---

## 2. 现有项目和工具

### 2.1 专用视频编辑DSL项目

#### SWML (Swimlane Markup Language)
- **类型**: 开源, AI导向的视频编辑标记语言
- **链接**: [GitHub - idreesaziz/GPT_Editor_MVP](https://github.com/idreesaziz/GPT_Editor_MVP)
- **文章**: [Why I Built My Own Markup Language for AI-Powered Video Editing](https://dev.to/idrees_a/why-i-built-my-own-markup-language-for-ai-powered-video-editing-5925)
- **描述**: JSON风格的声明式格式，专门为LLM生成视频编辑指令而设计。处理时序编排、图层叠加和时间控制。设计哲学是将"规划"、"合成"和"渲染"分离为独立步骤，每步可独立验证。
- **关键洞察**: LLM擅长生成结构化数据，不擅长生成可执行代码（如FFmpeg命令）。SWML作为中间表示，让LLM生成它擅长处理的格式，再由渲染引擎执行。
- **状态**: 仍在开发中，集成了Manim作为动画插件。

#### ytpdsl
- **类型**: 开源, 视频编辑DSL
- **链接**: [GitHub - AlexandreRio/ytpdsl](https://github.com/AlexandreRio/ytpdsl)
- **描述**: 基于视频库和数学运算的视频生成DSL。灵感来自 VideoGen2 项目。
- **定位**: 概念验证级别的视频编辑领域特定语言。

#### CodeVideo
- **类型**: 开源框架, 声明式视频生成
- **链接**: [codevideo.io](https://codevideo.io/) | [GitHub - codevideo](https://github.com/codevideo)
- **文章**: [Why Declarative Video Will Always Beat Generative Video](https://medium.com/codevideo/why-declarative-video-will-always-beat-generative-video-87f05c593b9f)
- **描述**: 基于"动作"(action)的声明式视频创建范式。将视频内容定义为一系列离散的教育性动作，存储在格式中立的JSON中。拥有完整的生态系统，包括TypeScript类型定义、虚拟IDE、虚拟文件浏览器、CLI工具、MCP服务器等。
- **核心理念**: 内容分离——定义"发生了什么"(what happens)，而非"如何显示"(how it's displayed)。

#### Code2Video (学术项目)
- **类型**: 学术研究项目
- **链接**: [GitHub - showlab/Code2Video](https://github.com/showlab/Code2Video) | [论文](https://arxiv.org/pdf/2510.01174)
- **描述**: 来自新加坡国立大学的代码中心范式教育视频生成框架。使用三个协作Agent（Planner、Coder、Critic）通过可执行的Manim代码生成高质量教育视频。

### 2.2 代码驱动的视频生成框架

#### Remotion (React)
- **类型**: 源码可用 (非完全开源), React视频框架
- **链接**: [remotion.dev](https://www.remotion.dev/) | [GitHub - remotion-dev/remotion](https://github.com/remotion-dev/remotion)
- **语言**: TypeScript/React
- **描述**: 用React组件定义视频的每一帧。将整个视频视为一个React组件树，利用CSS、Canvas、SVG、WebGL等Web技术。提供Remotion Player（嵌入式交互预览）和Remotion Lambda（云端规模化渲染）。
- **架构**: 核心思想是给你一个帧号和一个空白画布，用React渲染任意内容。参数化视频通过props驱动。
- **许可**: 个人/非营利/3人以下团队免费，大型组织需购买许可证。
- **生态**: Submagic等公司基于Remotion每月生产超过10万个视频。

#### Revideo (TypeScript, 基于Motion Canvas)
- **类型**: 开源 (MIT), TypeScript视频框架
- **链接**: [re.video](https://re.video/) | [GitHub - redotvideo/revideo](https://github.com/redotvideo/revideo) | [文档](https://redotvideo.github.io/)
- **背景**: Y Combinator支持的公司，从Motion Canvas fork而来
- **描述**: 用TypeScript创建视频模板，通过API渲染。提供React播放器组件实时预览。相比Motion Canvas增加了：无头渲染（可部署到Cloud Run等服务）、音频支持、更快的渲染速度、动态输入API。
- **定位**: 将Motion Canvas从独立应用转变为可构建视频编辑应用的库。

#### Motion Canvas
- **类型**: 开源, TypeScript动画库
- **链接**: [motioncanvas.io](https://motioncanvas.io/) | [GitHub - motion-canvas/motion-canvas](https://github.com/motion-canvas/motion-canvas)
- **描述**: 使用生成器(generators)编程动画的TypeScript库，配有实时预览编辑器。专为创建信息性矢量动画并与旁白同步而设计。

#### Editly (Node.js)
- **类型**: 开源, 声明式视频编辑
- **链接**: [GitHub - mifi/editly](https://github.com/mifi/editly)
- **描述**: 基于Node.js和FFmpeg的声明式NLE框架。通过JSON5规格定义视频合成。
- **规格结构**:
  ```json5
  {
    outPath: "output.mp4",
    width: 1920, height: 1080, fps: 30,
    clips: [
      {
        duration: 4,
        transition: { name: "fade", duration: 0.5 },
        layers: [
          { type: "video", path: "clip.mov" },
          { type: "title", text: "示例标题" }
        ]
      }
    ]
  }
  ```
- **图层类型**: video, audio, image, image-overlay, title, subtitle, title-background, news-title, slide-in-text, fill-color, radial-gradient, linear-gradient, canvas, fabric, gl (GLSL着色器)
- **转场**: 支持gl-transitions库预设 + directional-left/right/up/down + random
- **扩展性**: 支持自定义Canvas/Fabric.js代码和GLSL着色器

#### Etro (TypeScript, 浏览器)
- **类型**: 开源 (GPL-3.0), 浏览器视频编辑框架
- **链接**: [etrojs.dev](https://etrojs.dev/) | [GitHub - etro-js/etro](https://github.com/etro-js/etro)
- **描述**: TypeScript视频编辑框架，支持图层合成和GLSL硬件加速效果。支持播放、WebRTC流和录制。
- **核心API**: Movie对象 + 可组合的Layer/Effect类, 支持关键帧和自定义函数。
- **最近更新**: 2026年2月（活跃维护中）

#### Diffusion Studio Core
- **类型**: 开源, 浏览器视频引擎
- **链接**: [GitHub - diffusionstudio/core](https://github.com/diffusionstudio/core)
- **描述**: 基于WebCodecs API的快速浏览器视频合成引擎。面向时间线的NLE应用，支持视频、音频和图像工作负载。利用系统硬件加速进行编解码。

#### fframes (Rust)
- **类型**: 开源, Rust视频创建框架
- **链接**: [fframes.studio](https://fframes.studio/) | [Lib.rs](https://lib.rs/crates/fframes)
- **描述**: "第一个高性能开源视频创建框架"。与libav(FFmpeg)互操作，支持内存高效的帧渲染和GPU。最新更新: 2025年10月。
- **背景**: 作者花了约2年构建，认为"Rust可能是处理音视频的唯一正确方式"。

#### MoviePy (Python)
- **类型**: 开源 (MIT), Python视频编辑库
- **链接**: [GitHub - Zulko/moviepy](https://github.com/Zulko/moviepy) | [文档](https://zulko.github.io/moviepy/) | [PyPI](https://pypi.org/project/moviepy/)
- **版本**: v2.0 (2025年5月), 支持Python 3.9-3.11
- **描述**: 剪辑、拼接、标题插入、视频合成、视频处理和自定义特效。底层将媒体转换为NumPy数组，像素级可访问。
- **架构**: VideoFileClip/AudioFileClip作为核心类, CompositeVideoClip用于合成, FFmpeg用于编解码。
- **局限**: 不适合实时编辑，处理速度受FFmpeg和CPU限制。适合离线处理和预渲染。

### 2.3 商业API服务 (JSON-to-Video)

#### Shotstack
- **链接**: [shotstack.io](https://shotstack.io/) | [API文档](https://shotstack.io/docs/api/) | [JSON示例](https://github.com/shotstack/json-examples)
- **描述**: 云端视频编辑自动化平台。通过简单JSON描述视频编辑（图层、场景、转场、特效）。
- **JSON Schema结构**:
  ```json
  {
    "timeline": {
      "background": "#000000",
      "soundtrack": { "src": "url", "effect": "fadeOut" },
      "tracks": [{
        "clips": [{
          "asset": { "type": "title", "text": "Hello", "style": "minimal" },
          "start": 0,
          "length": 5,
          "transition": { "in": "fade", "out": "fade" },
          "effect": "zoomIn"
        }]
      }]
    },
    "output": { "format": "mp4", "resolution": "hd" }
  }
  ```
- **Asset类型**: title, video, image, audio, html, luma
- **SDK**: Python, Node.js, PHP, Ruby

#### Creatomate
- **链接**: [creatomate.com](https://creatomate.com/) | [RenderScript文档](https://creatomate.com/docs/json/introduction)
- **描述**: JSON声明式视频创建。RenderScript格式包含创建视频或图像所需的全部信息。
- **RenderScript结构**:
  ```json
  {
    "output_format": "mp4",
    "width": 1920,
    "height": 1080,
    "elements": [
      {
        "type": "image",
        "source": "https://...",
        "duration": 10,
        "y": "25%",
        "height": "50%"
      }
    ]
  }
  ```
- **特点**: 内置可视化编辑器可生成RenderScript代码。支持组合(Compositions)将多个元素分组。

#### JSON2Video
- **链接**: [json2video.com](https://json2video.com/) | [API参考](https://json2video.com/docs/v2/api-reference) | [JSON语法](https://json2video.com/docs/v2/api-reference/json-syntax)
- **描述**: 通过Movie JSON Schema定义视频。层级结构: Movie -> Scenes -> Elements。
- **元素类型**: Image, Video, Text, Component, Audio, Audiogram, Voice (TTS), Subtitles
- **高级特性**: 变量和表达式、条件渲染、动态场景生成
- **SDK**: PHP, Node.js, Python, Go

#### Plainly
- **链接**: [plainlyvideos.com](https://www.plainlyvideos.com/)
- **描述**: 基于After Effects的API驱动参数化视频生成。比Creatomate更强大但需要更多开发者介入和AE知识。

### 2.4 传统NLE脚本系统

#### Adobe ExtendScript (After Effects / Premiere Pro)
- **链接**: [AE脚本指南](https://ae-scripting.docsforadobe.dev/) | [PPro脚本指南](https://ppro-scripting.docsforadobe.dev/)
- **语言**: ExtendScript (ECMAScript 3扩展), .jsx/.jsxbin
- **能力**: AE中可自动化复杂动画任务、创建自定义效果、增强工作流。PPro中可批量导出、自动剪切、批量编辑。
- **局限**: 基于旧版JavaScript (ES3), 不支持let/const、箭头函数、Promise等现代特性。
- **迁移**: Adobe正在向UXP (Unified Extensibility Platform)过渡。ExtendScript支持计划到2026年9月。
- **无头模式**: 不支持（仅限GUI交互或通过CEP panel）。

#### DaVinci Resolve 脚本API
- **链接**: [官方Wiki](https://wiki.dvresolve.com/developer-docs/scripting-api) | [非官方文档](https://deric.github.io/DaVinciResolve-API-Docs/) | [ResolveDevDoc](https://resolvedevdoc.readthedocs.io/)
- **语言**: Lua 5.1, Python 2.7/3.6+
- **能力**: 通过FusionScript自动化重复性任务、自定义行为、扩展功能、与第三方应用交换数据。
- **无头模式**: 支持 `-nogui` 命令行选项，脚本API完全正常工作。
- **关键入口**: Fusion(), GetMediaStorage(), GetProjectManager(), OpenPage()
- **注意**: Fuse不能用Python编写, EventScript只支持Lua。

---

## 3. 相关技术生态

### 3.1 交换格式与标准

#### OpenTimelineIO (OTIO)
- **链接**: [GitHub - AcademySoftwareFoundation/OpenTimelineIO](https://github.com/AcademySoftwareFoundation/OpenTimelineIO) | [文档](https://opentimelineio.readthedocs.io/) | [格式规范](https://github.com/AcademySoftwareFoundation/OpenTimelineIO/blob/main/docs/tutorials/otio-file-format-specification.md)
- **组织**: Academy Software Foundation (ASWF)
- **描述**: 编辑时间线信息的开源API和交换格式。可以理解为现代版的EDL。
- **数据模型** (类树形AST结构):
  ```
  Timeline
  └── tracks: Stack
      ├── Track (video)
      │   ├── Clip (引用外部媒体)
      │   ├── Gap
      │   ├── Transition
      │   └── Clip
      └── Track (audio)
          ├── Clip
          └── Gap
  ```
- **文件格式**: JSON序列化, `.otio`扩展名, 推荐缩进格式以保持人类可读性
- **Schema版本控制**: 每个数据类型独立版本 (如 `"OTIO_SCHEMA": "Timeline.1"`)
- **适配器插件**: Final Cut Pro XML, AAF, CMX 3600 EDL等
- **文件包**: OTIOZ (.zip) / OTIOD (目录) 可打包切割信息和媒体
- **关键特性**: 不支持实例化(instancing), 同一clip多次出现时为独立副本

#### EDL (Edit Decision List) - CMX 3600
- **链接**: [Wikipedia](https://en.wikipedia.org/wiki/Edit_decision_list) | [CMX 3600规范](https://xmil.biz/EDL-X/CMX3600.pdf) | [如何阅读EDL](https://www.niwa.nu/2013/05/how-to-read-an-edl/)
- **描述**: 最常见的EDL格式，1970年代设计，仍是编辑系统间的"最低公约数"。
- **格式**: 纯ASCII文本文件
- **结构**: 每行一个事件，包含: 事件号(max 999), 源卷盘名(max 8字符), 轨道类型(V/A), 转场类型(C/D/W), 4个时间码(源入/出, 录入/出)
- **局限**: 最多999个事件, 1个视频轨, 4个音频轨, 仅支持基本功能(切、溶解、SMPTE擦除、变速)
- **Python解析**: [pycmx](https://github.com/iluvcapra/pycmx)

#### AAF (Advanced Authoring Format)
- **链接**: [Wikipedia](https://en.wikipedia.org/wiki/Advanced_Authoring_Format) | [对象规范](https://aafassociation.org/specs/object_spec.html) | [Library of Congress](https://www.loc.gov/preservation/digital/formats/fdd/fdd000004.shtml)
- **组织**: Advanced Media Workflow Association (AMWA)
- **描述**: 面向专业后期制作和创作环境的跨平台数据交换文件格式。
- **数据类型**: Essence Data (音视频/图像/图形/文本/动画) + Metadata (组合和修改信息)
- **与MXF的关系**: MXF是AAF数据模型的子类型，遵循零偏差策略
- **底层存储**: 使用Microsoft Structured Storage
- **行业支持**: BBC, CNN, Warner Bros., Fox, Avid, Adobe, Sony等

### 3.2 多媒体框架

#### FFmpeg (命令行 / filtergraph DSL)
- **链接**: [ffmpeg.org](https://ffmpeg.org/) | [滤镜文档](https://ffmpeg.org/ffmpeg-filters.html) | [FilteringGuide](https://trac.ffmpeg.org/wiki/FilteringGuide) | [FFmpeg Explorer](https://lav.io/notes/ffmpeg-explorer/)
- **描述**: 领先的多媒体框架。其`filter_complex`实际上是一种图形化的DSL。
- **filtergraph语法**: 有向图的连接滤镜, 用`,`分隔滤镜链, 用`;`分隔滤镜链组, 用`[]`标记链接标签
- **示例**: `[0:v]scale=1280:720[scaled];[scaled]overlay=10:10[out]`
- **程序化工具**:
  - [FFmpeg Explorer](https://lav.io/notes/ffmpeg-explorer/) - 节点编辑器生成命令
  - [ffmpeg-filter_graph](https://github.com/sangster/ffmpeg-filter_graph) - Ruby gem用于构建复杂filtergraph
- **AI适配性问题**: 转义复杂度高（3层转义）、大型filtergraph易出现随机难调试错误、语法对LLM不友好

#### MLT Framework
- **链接**: [mltframework.org](https://www.mltframework.org/) | [GitHub - mltframework/mlt](https://github.com/mltframework/mlt)
- **描述**: 面向电视广播的开源多媒体框架。用于Kdenlive和Shotcut的项目文件格式(MLT XML)。
- **XML Schema**: 支持playlist, tractor, track, producer/chain, filter, transition等元素
- **语言绑定**: C++, Java, Lua, Perl, PHP, Python, Ruby, Tcl
- **服务端渲染**: 支持接受MLT XML项目文件的渲染服务器

#### GStreamer / GES (GStreamer Editing Services)
- **链接**: [GStreamer](https://gstreamer.freedesktop.org/) | [GES文档](https://gstreamer.freedesktop.org/documentation/gst-editing-services/)
- **描述**: 基于管道(pipeline)的多媒体框架。GES是高层编辑库。
- **管道语法**: 元素用`!`连接, 属性用`property=value`设置
- **GES概念**: Timeline(GstElement) -> Layers(用户可见的clip排列) -> Tracks(输出流)
- **编辑能力**: GESPipeline支持预览/播放(playsink)和渲染(encodebin)模式切换

### 3.3 帧服务器脚本系统

#### AviSynth / AviSynth+
- **链接**: [Wikipedia](https://en.wikipedia.org/wiki/AviSynth)
- **描述**: 帧服务器, 使用自定义的数据流DSL。调用程序请求帧, 脚本服务帧。
- **语言特点**: 数据流语言(dataflow language), 描述数据在操作间流动的有向图。缺乏部分过程式控制结构，但包含变量、类型、条件和复杂表达式。
- **AviSynth+**: fork版本, 增加64位支持、多线程、深色彩空间、循环等新控制流结构。

#### VapourSynth
- **链接**: [vapoursynth.com](https://www.vapoursynth.com/) | [HackerNews讨论](https://news.ycombinator.com/item?id=38613938)
- **描述**: AviSynth的现代重写。使用Python而非自定义DSL。
- **优势**: 帧级多线程、通用化色彩空间、每帧元数据、Python标准库可用
- **互操作**: 支持许多AviSynth插件, VapourSource允许跨框架使用

### 3.4 节点式合成脚本

#### Natron (开源After Effects/Nuke替代)
- **链接**: [natrongithub.github.io](https://natrongithub.github.io/) | [GitHub - NatronGitHub/Natron](https://github.com/NatronGitHub/Natron) | [Python脚本仓库](https://github.com/NatronGitHub/natron-python-scripting)
- **描述**: 免费开源的节点式合成软件。功能类似AE/Nuke/Fusion。支持Python 2.7脚本。
- **脚本能力**: 用户自定义Python回调, PySide GUI扩展, PyPlug自定义节点(等同Nuke Gizmos)
- **命令行渲染**: 支持无显示器的后台渲染(渲染农场), 可通过Python脚本设置图形并渲染

#### Nuke (The Foundry) / Blender 合成器
- **描述**: 工业级节点式合成软件。Nuke使用Python脚本和TCL进行自动化, 内部节点图可以序列化为.nk脚本文件。
- **Blender合成器**: Blender内置的节点式合成系统, 可通过Python (bpy) 程序化构建节点图。

---

## 4. 学术研究

### 4.1 LAVE: LLM-Powered Agent Assistance and Language Augmentation for Video Editing
- **发表**: ACM IUI 2024
- **作者**: 多伦多大学, Meta, UCSD
- **链接**: [论文](https://arxiv.org/html/2402.10294v1) | [项目页面](https://www.dgp.toronto.edu/~bryanw/lave/) | [ACM](https://dl.acm.org/doi/10.1145/3640543.3645143)
- **核心**: 自动为用户素材生成语言描述, 作为LLM处理视频和辅助编辑的基础。Agent规划并执行相关操作。使用结构化JSON输出: `{"segment": ["start", "end", "rationale"]}`。
- **关键发现**: 将视频内容转化为语言描述是连接LLM与视频编辑操作的桥梁。

### 4.2 From Shots to Stories: LLM-Assisted Video Editing with Unified Language Representations
- **发表**: 2025年5月 (arXiv)
- **链接**: [arXiv:2505.12237](https://arxiv.org/abs/2505.12237)
- **核心**: 首个系统性研究LLM在视频编辑中应用的论文。提出 **L-Storyboard** 中间表示, 将离散视频镜头转化为适合LLM处理的结构化语言描述。
- **任务分类**: 收敛型任务(Convergent) vs 发散型任务(Divergent)
- **关键创新**: StoryFlow策略, 将发散型多路径推理转化为收敛型选择机制。

### 4.3 Prompt-Driven Agentic Video Editing System
- **发表**: 2025年9月 (arXiv)
- **链接**: [arXiv:2509.16811](https://arxiv.org/html/2509.16811v1)
- **核心**: 面向长篇叙事性媒体的自主理解和编辑。维护持久的、可编辑的叙事结构表示。

### 4.4 VideoAgent: All-in-One Agentic Framework
- **链接**: [GitHub - HKUDS/VideoAgent](https://github.com/HKUDS/VideoAgent)
- **核心**: 通过纯对话式AI实现视频交互和创作的全能Agent框架。

### 4.5 ExpressEdit: Video Editing with Natural Language and Sketching
- **发表**: ACM IUI 2024
- **链接**: [ACM](https://dl.acm.org/doi/10.1145/3640543.3645164)
- **核心**: 结合自然语言和草图的视频编辑界面。

### 4.6 M3L: Language-based Video Editing via Multi-Modal Multi-Level Transformers
- **发表**: 2021
- **链接**: [arXiv:2104.01122](https://arxiv.org/abs/2104.01122)
- **核心**: 引入"基于语言的视频编辑"(LBVE)任务, 通过文本指令引导源视频到目标视频的编辑。

### 4.7 LLM-Grounded Video Diffusion (LVD)
- **链接**: [项目页面](https://llm-grounded-video-diffusion.github.io/)
- **核心**: LLM作为时空规划器生成"动态场景布局"(DSL, 恰好也缩写为DSL), 包含跨帧链接的对象边界框。两阶段均免训练。

### 4.8 Edit3K: Universal Representation Learning for Video Editing Components
- **链接**: [arXiv:2403.16048](https://arxiv.org/abs/2403.16048)
- **核心**: 分析主流视频创作流水线, 归纳6种主要编辑组件类型: video effects, animation, transition, filter, sticker, text。

---

## 5. 现有DSL/Schema设计对比分析

| 特性 | Shotstack | Creatomate | JSON2Video | Editly | OTIO | EDL (CMX3600) | MLT XML |
|------|-----------|------------|------------|--------|------|---------------|---------|
| **格式** | JSON | JSON | JSON | JSON5 | JSON | ASCII文本 | XML |
| **层级** | Timeline>Tracks>Clips | Elements/Compositions | Movie>Scenes>Elements | Clips>Layers | Timeline>Stack>Tracks>Items | 事件列表 | Playlist/Tractor/Track |
| **时间模型** | 绝对时间(start+length) | 绝对时间(duration) | 场景级别 | 片段级别(duration) | 相对时间(RationalTime) | SMPTE时间码 | 灵活 |
| **转场** | in/out属性 | 动画属性 | 场景间 | transition对象 | Transition对象 | C/D/W代码 | transition元素 |
| **特效** | effect字符串 | 滤镜+动画 | 内置动画 | 图层类型 | Effect对象 | 无 | filter元素 |
| **关键帧** | 有限 | 完整支持 | 有限 | 通过函数 | 无(静态) | 无 | 无 |
| **音频** | soundtrack | 元素级别 | Audio元素 | 多轨道+混音 | Track级别 | 4轨道 | 完整 |
| **嵌套合成** | 无 | Compositions | 无 | 无 | Stack嵌套 | 无 | Tractor |
| **人类可读性** | 高 | 高 | 高 | 高 | 高 | 中 | 中 |
| **LLM友好度** | 高 | 高 | 高 | 高 | 中高 | 低 | 低 |
| **渲染** | 云端 | 云端 | 云端 | 本地(FFmpeg) | 无(纯数据) | 无(纯数据) | 本地 |

---

## 6. 语言设计关键考量

### 6.1 基本原语 (Primitives)

根据对现有工具的分析，一个视频编辑DSL需要以下核心原语:

```
Timeline          -- 顶层容器, 定义全局属性(分辨率、帧率、时长)
├── Track         -- 图层轨道, 定义堆叠顺序
│   ├── Clip      -- 时间线上的媒体片段
│   │   ├── source     -- 媒体引用(文件路径/URL)
│   │   ├── start      -- 在时间线上的起始时间
│   │   ├── duration   -- 持续时长
│   │   ├── trim_in    -- 源媒体裁剪入点
│   │   └── trim_out   -- 源媒体裁剪出点
│   ├── Gap       -- 空白间隔
│   └── Transition -- 转场效果
├── Effect        -- 应用于clip/track/timeline的效果
│   ├── type           -- 效果类型
│   ├── parameters     -- 参数
│   └── keyframes      -- 关键帧
├── Text          -- 文字覆盖层
├── Audio         -- 音频轨道/配乐
└── Output        -- 输出配置(格式、编码、分辨率)
```

### 6.2 时间线表示

两种主要模式:

1. **绝对时间模型** (Shotstack风格): 每个clip有明确的`start`和`length`
   - 优点: 直观, LLM容易生成
   - 缺点: 调整一个clip影响后续所有clip时需要手动更新

2. **相对/顺序模型** (Editly/OTIO风格): clip按顺序排列, 时长决定位置
   - 优点: 添加/删除clip时自动重排
   - 缺点: 精确定位需要额外计算

**推荐**: 混合模型 — 默认顺序排列, 支持可选的绝对定位覆盖。

### 6.3 音视频同步

- **OTIO方法**: RationalTime (有理数时间表示, 避免浮点精度问题)
- **实践建议**:
  - 使用帧号或有理数而非浮点秒数表示精确时间
  - 音频和视频应在同一时间坐标系中
  - 支持明确的音视频链接关系 (linked clips)

### 6.4 特效参数与关键帧

```json
{
  "effect": "opacity",
  "keyframes": [
    { "time": 0.0, "value": 0.0, "easing": "ease-in" },
    { "time": 1.0, "value": 1.0, "easing": "linear" },
    { "time": 4.0, "value": 1.0 },
    { "time": 5.0, "value": 0.0, "easing": "ease-out" }
  ]
}
```

关键设计点:
- 时间可以是绝对时间或相对于clip的时间
- 支持缓动函数(easing): linear, ease-in, ease-out, ease-in-out, cubic-bezier
- 特效应可堆叠和排序

### 6.5 合成与图层

- **Z-order**: 轨道顺序决定叠放顺序 (上层覆盖下层)
- **混合模式**: normal, multiply, screen, overlay等
- **遮罩**: alpha通道, luma键, 色度键
- **变换**: position(x,y), scale, rotation, anchor point

### 6.6 文字/图形覆盖

- 应支持内联样式定义 (字体、大小、颜色、对齐)
- 动画应可独立控制 (入场、持续、退场)
- 考虑支持HTML/CSS子集以获得丰富的排版能力

### 6.7 导出/渲染流水线

```json
{
  "output": {
    "format": "mp4",
    "codec": { "video": "h264", "audio": "aac" },
    "resolution": { "width": 1920, "height": 1080 },
    "fps": 30,
    "bitrate": { "video": "8M", "audio": "192k" },
    "quality": "high"
  }
}
```

---

## 7. AI Agent适配性考量

### 7.1 LLM生成友好的语言特性

基于SWML、LAVE、L-Storyboard等项目的经验:

1. **JSON是最佳选择**: LLM擅长生成结构化JSON, 而非FFmpeg命令或代码
2. **扁平化优于深度嵌套**: 减少嵌套层级, 使用引用而非嵌套
3. **枚举值优于自由文本**: 转场类型、效果类型等使用预定义枚举
4. **有意义的默认值**: 省略的属性应有合理默认值
5. **独立验证**: 每个组件应能独立验证, 不依赖全局状态
6. **推理字段**: 在Schema中包含`rationale`字段, 让LLM解释其编辑决策

### 7.2 声明式 vs 命令式

| 方面 | 声明式 (推荐) | 命令式 |
|------|-------------|--------|
| **描述方式** | "视频应该是什么样" | "如何一步步构建视频" |
| **LLM适配性** | 高 — 描述最终状态 | 低 — 需要记住执行上下文 |
| **示例** | Shotstack JSON, Editly | FFmpeg命令, MoviePy代码 |
| **验证** | 静态验证Schema | 需要执行才能验证 |
| **可预测性** | 高 — 相同输入相同输出 | 低 — 依赖执行环境 |
| **表达力** | 有限但足够 | 无限但复杂 |

**结论**: 对于AI Agent, **声明式方法显著优于命令式**。核心原因:
- LLM生成声明式JSON的成功率远高于生成正确的命令序列
- 声明式描述可以通过JSON Schema在生成前/后进行验证
- 支持约束解码(constrained decoding)工具如Outlines保证有效输出

### 7.3 错误处理与验证

1. **JSON Schema验证**: 提供严格的JSON Schema, 利用LLM的结构化输出能力
2. **分层验证**:
   - 语法层: JSON格式是否正确
   - 语义层: 引用的资源是否存在, 时间范围是否合理
   - 逻辑层: 时间线是否有冲突, 图层是否正确叠放
3. **错误消息设计**: 包含位置信息和修复建议, 方便LLM自我修正
4. **渐进式构建**: 支持增量编辑, 而非每次重新生成全部

### 7.4 关键设计原则

来自实践经验的总结:

1. **中间表示模式** (SWML的核心洞察):
   ```
   自然语言 → [LLM] → 结构化中间表示(JSON/DSL) → [验证] → [渲染引擎] → 视频
   ```

2. **三阶段分离**:
   - **规划**: LLM理解意图, 生成编辑计划
   - **合成**: 将计划转化为精确的DSL描述
   - **渲染**: 将DSL描述送入渲染引擎

3. **避免让LLM直接生成**:
   - FFmpeg命令 (转义复杂, 语法脆弱)
   - Python/JS代码 (运行时错误多, 难以验证)
   - 像素级参数 (LLM对数值不敏感)

4. **应该让LLM生成**:
   - 高层编辑意图的结构化表示
   - 从预定义选项中选择 (转场类型、效果类型)
   - 相对化的时间和位置描述 (如 "25%", "center", "after clip X")

---

## 8. 结论与建议

### 8.1 现状总结

"Video Editing as Code" 生态当前处于快速发展但仍然碎片化的阶段:

- **交换格式**: OTIO是最成熟的标准, 但主要面向专业后期, 对AI Agent不够友好
- **商业JSON API**: Shotstack/Creatomate/JSON2Video提供了成熟的JSON-to-Video方案, 但各自Schema不互通
- **代码框架**: Remotion/Revideo/Editly各有优势, 但需要编写代码而非纯声明式
- **AI专用**: SWML是目前唯一明确为AI Agent设计的视频编辑标记语言, 但仍在早期

### 8.2 设计新DSL的建议

如果要设计一个面向AI Agent的视频编辑DSL, 建议:

1. **格式**: JSON (不是YAML, 不是XML, 不是自定义语法)
   - LLM对JSON的生成能力最强
   - 工具链最成熟 (JSON Schema验证、结构化输出)

2. **数据模型**: 借鉴OTIO的树形结构 + Shotstack的简洁性
   - Timeline > Tracks > Clips/Gaps/Transitions
   - 支持全局属性继承和覆盖

3. **时间系统**: 支持多种时间表示
   - 帧号: `{"frame": 150}`
   - 秒数: `{"seconds": 5.0}`
   - 时间码: `{"timecode": "00:00:05:00"}`
   - 相对时间: `{"after": "clip_id", "offset": 1.0}`

4. **抽象层次**: 提供高/中/低三层抽象
   - 高层: `{"action": "cut_to_music_beat"}` (语义级别)
   - 中层: `{"clip": "intro.mp4", "start": 0, "duration": 5}` (结构级别)
   - 低层: `{"filter": "colorbalance", "rs": 0.3}` (参数级别)

5. **AI特性**:
   - 每个操作包含可选的`rationale`字段
   - 支持约束声明 (如 "total_duration <= 60s")
   - 支持模板/变量系统减少重复
   - 提供丰富的JSON Schema用于约束解码

### 8.3 推荐参考实现优先级

1. **OTIO** — 学习其数据模型和时间系统设计
2. **Shotstack JSON Schema** — 学习其简洁的API设计
3. **Editly** — 学习其声明式规格和图层系统
4. **SWML/GPT_Editor_MVP** — 学习其AI-first的设计哲学
5. **Creatomate RenderScript** — 学习其关键帧和动画系统

---

## 参考链接汇总

### 开源项目
- [OpenTimelineIO](https://github.com/AcademySoftwareFoundation/OpenTimelineIO)
- [Remotion](https://github.com/remotion-dev/remotion)
- [Revideo](https://github.com/redotvideo/revideo)
- [Motion Canvas](https://github.com/motion-canvas/motion-canvas)
- [Editly](https://github.com/mifi/editly)
- [Etro](https://github.com/etro-js/etro)
- [MoviePy](https://github.com/Zulko/moviepy)
- [Diffusion Studio Core](https://github.com/diffusionstudio/core)
- [fframes](https://fframes.studio/)
- [GPT_Editor_MVP (SWML)](https://github.com/idreesaziz/GPT_Editor_MVP)
- [CodeVideo](https://github.com/codevideo)
- [Code2Video](https://github.com/showlab/Code2Video)
- [ytpdsl](https://github.com/AlexandreRio/ytpdsl)
- [Natron](https://github.com/NatronGitHub/Natron)
- [MLT Framework](https://github.com/mltframework/mlt)
- [VideoAgent](https://github.com/HKUDS/VideoAgent)
- [ai-video-editor](https://github.com/mitch7w/ai-video-editor)
- [pycmx](https://github.com/iluvcapra/pycmx)
- [VapourSynth](https://www.vapoursynth.com/)

### 商业服务
- [Shotstack](https://shotstack.io/)
- [Creatomate](https://creatomate.com/)
- [JSON2Video](https://json2video.com/)
- [Plainly](https://www.plainlyvideos.com/)

### 学术论文
- [LAVE (ACM IUI 2024)](https://dl.acm.org/doi/10.1145/3640543.3645143)
- [From Shots to Stories (2025)](https://arxiv.org/abs/2505.12237)
- [ExpressEdit (ACM IUI 2024)](https://dl.acm.org/doi/10.1145/3640543.3645164)
- [M3L (2021)](https://arxiv.org/abs/2104.01122)
- [Edit3K (2024)](https://arxiv.org/abs/2403.16048)
- [Code2Video (2025)](https://arxiv.org/pdf/2510.01174)
- [Prompt-Driven Agentic Video Editing (2025)](https://arxiv.org/html/2509.16811v1)

### 文档与规范
- [OTIO格式规范](https://github.com/AcademySoftwareFoundation/OpenTimelineIO/blob/main/docs/tutorials/otio-file-format-specification.md)
- [Shotstack API文档](https://shotstack.io/docs/api/)
- [Creatomate RenderScript](https://creatomate.com/docs/json/introduction)
- [JSON2Video JSON语法](https://json2video.com/docs/v2/api-reference/json-syntax)
- [FFmpeg滤镜文档](https://ffmpeg.org/ffmpeg-filters.html)
- [DaVinci Resolve脚本API](https://wiki.dvresolve.com/developer-docs/scripting-api)
- [AE脚本指南](https://ae-scripting.docsforadobe.dev/)
- [Premiere Pro脚本指南](https://ppro-scripting.docsforadobe.dev/)
- [GStreamer Editing Services](https://gstreamer.freedesktop.org/documentation/gst-editing-services/)

### 文章与讨论
- [Why I Built My Own Markup Language for AI-Powered Video Editing](https://dev.to/idrees_a/why-i-built-my-own-markup-language-for-ai-powered-video-editing-5925)
- [Why Declarative Video Will Always Beat Generative Video](https://medium.com/codevideo/why-declarative-video-will-always-beat-generative-video-87f05c593b9f)
- [LLM-Powered Video Editing via Natural Language Scripting (Toronto AI Tinkerers)](https://toronto.aitinkerers.org/talks/rsvp__O4FHg-kOBU)
- [Claude + Remotion AI视频创作](https://dev.to/mayu2008/new-clauderemotion-to-create-amazing-videos-using-ai-37bp)
