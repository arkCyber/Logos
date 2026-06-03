/**
 * 原生菜单栏模块
 * 提供操作系统级别的原生菜单栏功能
 * 包括文件、编辑、视图、插入、格式、工具、帮助等菜单
 */

use tauri::{AppHandle, menu::{Menu, MenuBuilder, SubmenuBuilder, MenuItemBuilder, PredefinedMenuItem}, Runtime};

/**
 * 创建原生菜单栏
 */
pub fn create_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, String> {
    let menu = MenuBuilder::new(app)
        // 文件菜单
        .items(&[
            &SubmenuBuilder::new(app, "文件")
                .items(&[
                    &MenuItemBuilder::new("新建")
                        .id("file_new")
                        .accelerator("CmdOrControl+N")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("打开")
                        .id("file_open")
                        .accelerator("CmdOrControl+O")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("保存")
                        .id("file_save")
                        .accelerator("CmdOrControl+S")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("另存为")
                        .id("file_save_as")
                        .accelerator("CmdOrControl+Shift+S")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("导出 PDF")
                        .id("file_export_pdf")
                        .accelerator("CmdOrControl+P")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("导出 PNG")
                        .id("file_export_png")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("Export SVG (Typst)")
                        .id("file_export_svg_typst")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("Export SVG (HTML)")
                        .id("file_export_svg_html")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("导出 Typst")
                        .id("file_export_typst")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("导出 DOCX")
                        .id("file_export_docx")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("打印")
                        .id("file_print")
                        .accelerator("CmdOrControl+P")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("退出")
                        .id("file_quit")
                        .accelerator("CmdOrControl+Q")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 编辑菜单
            &SubmenuBuilder::new(app, "编辑")
                .items(&[
                    &MenuItemBuilder::new("撤销")
                        .id("edit_undo")
                        .accelerator("CmdOrControl+Z")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("重做")
                        .id("edit_redo")
                        .accelerator("CmdOrControl+Shift+Z")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("剪切")
                        .id("edit_cut")
                        .accelerator("CmdOrControl+X")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("复制")
                        .id("edit_copy")
                        .accelerator("CmdOrControl+C")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("粘贴")
                        .id("edit_paste")
                        .accelerator("CmdOrControl+V")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("全选")
                        .id("edit_select_all")
                        .accelerator("CmdOrControl+A")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("查找")
                        .id("edit_find")
                        .accelerator("CmdOrControl+F")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("替换")
                        .id("edit_replace")
                        .accelerator("CmdOrControl+H")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 视图菜单
            &SubmenuBuilder::new(app, "视图")
                .items(&[
                    &MenuItemBuilder::new("全屏")
                        .id("view_fullscreen")
                        .accelerator("CmdOrControl+Shift+F")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("缩放")
                        .id("view_zoom")
                        .accelerator("CmdOrControl++")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("缩小")
                        .id("view_zoom_out")
                        .accelerator("CmdOrControl+-")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("重置缩放")
                        .id("view_zoom_reset")
                        .accelerator("CmdOrControl+0")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("显示/隐藏侧边栏")
                        .id("view_sidebar")
                        .accelerator("CmdOrControl+B")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("显示/隐藏状态栏")
                        .id("view_statusbar")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("Typst 预览")
                        .id("view_typst_preview")
                        .accelerator("CmdOrControl+R")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 插入菜单
            &SubmenuBuilder::new(app, "插入")
                .items(&[
                    &MenuItemBuilder::new("图片")
                        .id("insert_image")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("表格")
                        .id("insert_table")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("链接")
                        .id("insert_link")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("代码块")
                        .id("insert_code_block")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("数学公式")
                        .id("insert_formula")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("Emoji")
                        .id("insert_emoji")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 格式菜单
            &SubmenuBuilder::new(app, "格式")
                .items(&[
                    &MenuItemBuilder::new("加粗")
                        .id("format_bold")
                        .accelerator("CmdOrControl+B")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("斜体")
                        .id("format_italic")
                        .accelerator("CmdOrControl+I")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("下划线")
                        .id("format_underline")
                        .accelerator("CmdOrControl+U")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("删除线")
                        .id("format_strikethrough")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("上标")
                        .id("format_superscript")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("下标")
                        .id("format_subscript")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("对齐方式")
                        .id("format_align")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("行距")
                        .id("format_line_spacing")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("样式")
                        .id("format_style")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 工具菜单
            &SubmenuBuilder::new(app, "工具")
                .items(&[
                    &MenuItemBuilder::new("拼写检查")
                        .id("tools_spell_check")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("字数统计")
                        .id("tools_word_count")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("AI 润色")
                        .id("tools_ai_polish")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("AI 扩写")
                        .id("tools_ai_expand")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("AI 翻译")
                        .id("tools_ai_translate")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("Typst 包管理器")
                        .id("tools_typst_packages")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("设置")
                        .id("tools_settings")
                        .accelerator("CmdOrControl+,")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
            // 帮助菜单
            &SubmenuBuilder::new(app, "帮助")
                .items(&[
                    &MenuItemBuilder::new("用户指南")
                        .id("help_user_guide")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("快捷键")
                        .id("help_shortcuts")
                        .accelerator("CmdOrControl+?")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("API 文档")
                        .id("help_api_docs")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("检查更新")
                        .id("help_check_updates")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &MenuItemBuilder::new("反馈问题")
                        .id("help_feedback")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                    &PredefinedMenuItem::separator(app).map_err(|e| format!("Failed to create separator: {}", e))?,
                    &MenuItemBuilder::new("关于")
                        .id("help_about")
                        .build(app)
                        .map_err(|e| format!("Failed to create menu item: {}", e))?,
                ])
                .build()
                .map_err(|e| format!("Failed to create submenu: {}", e))?,
        ])
        .build()
        .map_err(|e| format!("Failed to create menu: {}", e))?;

    Ok(menu)
}
