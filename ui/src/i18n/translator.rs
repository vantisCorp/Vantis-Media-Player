//! Translation engine
//! 
//! Handles loading and retrieving translations for the current language.

use super::Language;
use std::collections::HashMap;

/// Translation storage
#[derive(Debug, Clone)]
pub struct Translator {
    /// Current language
    language: Language,
    /// Translation strings for current language
    translations: HashMap<String, String>,
    /// Fallback translations (English)
    fallback: HashMap<String, String>,
}

impl Translator {
    /// Create a new translator for the specified language
    pub fn new(language: Language) -> Self {
        Self {
            language,
            translations: load_translations(language),
            fallback: load_translations(Language::English),
        }
    }
    
    /// Translate a key to the current language
    /// 
    /// Supports nested keys like "common.buttons.play"
    pub fn translate(&self, key: &str) -> String {
        // Try current language first
        if let Some(text) = self.translations.get(key) {
            return text.clone();
        }
        
        // Try fallback (English)
        if let Some(text) = self.fallback.get(key) {
            return text.clone();
        }
        
        // Return the key itself if not found
        format!("⚠️ {}", key)
    }
    
    /// Check if a translation exists
    pub fn has(&self, key: &str) -> bool {
        self.translations.contains_key(key) || self.fallback.contains_key(key)
    }
    
    /// Get all translations for a namespace (prefix)
    /// 
    /// Example: get_namespace("common.buttons") returns all keys starting with "common.buttons."
    pub fn get_namespace(&self, prefix: &str) -> Vec<(&str, &str)> {
        let prefix_with_dot = format!("{}.", prefix);
        self.translations
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix_with_dot))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
    
    /// Get the current language
    pub fn language(&self) -> Language {
        self.language
    }
}

/// Load translations for a language
fn load_translations(language: Language) -> HashMap<String, String> {
    match language {
        Language::English => english_translations(),
        Language::Polish => polish_translations(),
        Language::German => german_translations(),
        Language::Chinese => chinese_translations(),
        Language::Russian => russian_translations(),
        Language::Korean => korean_translations(),
        Language::Spanish => spanish_translations(),
        Language::French => french_translations(),
    }
}

// ============================================================================
// ENGLISH TRANSLATIONS (Base)
// ============================================================================

fn english_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Play");
    m.insert("common.pause", "Pause");
    m.insert("common.stop", "Stop");
    m.insert("common.skip", "Skip");
    m.insert("common.previous", "Previous");
    m.insert("common.next", "Next");
    m.insert("common.volume", "Volume");
    m.insert("common.mute", "Mute");
    m.insert("common.unmute", "Unmute");
    m.insert("common.fullscreen", "Fullscreen");
    m.insert("common.exit_fullscreen", "Exit Fullscreen");
    m.insert("common.settings", "Settings");
    m.insert("common.close", "Close");
    m.insert("common.save", "Save");
    m.insert("common.cancel", "Cancel");
    m.insert("common.confirm", "Confirm");
    m.insert("common.delete", "Delete");
    m.insert("common.edit", "Edit");
    m.insert("common.search", "Search");
    m.insert("common.filter", "Filter");
    m.insert("common.sort", "Sort");
    m.insert("common.refresh", "Refresh");
    m.insert("common.loading", "Loading...");
    m.insert("common.error", "Error");
    m.insert("common.success", "Success");
    m.insert("common.warning", "Warning");
    m.insert("common.info", "Info");
    
    // Navigation
    m.insert("nav.home", "Home");
    m.insert("nav.library", "Library");
    m.insert("nav.discover", "Discover");
    m.insert("nav.search", "Search");
    m.insert("nav.downloads", "Downloads");
    m.insert("nav.settings", "Settings");
    m.insert("nav.account", "Account");
    m.insert("nav.help", "Help");
    m.insert("nav.about", "About");
    
    // Media player
    m.insert("player.play", "Play");
    m.insert("player.pause", "Pause");
    m.insert("player.stop", "Stop");
    m.insert("player.skip_forward", "Skip Forward");
    m.insert("player.skip_backward", "Skip Backward");
    m.insert("player.speed", "Playback Speed");
    m.insert("player.quality", "Quality");
    m.insert("player.subtitles", "Subtitles");
    m.insert("player.audio_track", "Audio Track");
    m.insert("player.subtitle_none", "None");
    m.insert("player.subtitle_search", "Search for subtitles...");
    m.insert("player.subtitle_download", "Download Subtitles");
    m.insert("player.subtitle_offset", "Subtitle Offset");
    m.insert("player.screenshot", "Screenshot");
    m.insert("player.record", "Record");
    m.insert("player.loop", "Loop");
    m.insert("player.shuffle", "Shuffle");
    m.insert("player.picture_in_picture", "Picture-in-Picture");
    
    // Library
    m.insert("library.title", "Library");
    m.insert("library.empty", "Your library is empty");
    m.insert("library.empty_hint", "Add media files to start watching");
    m.insert("library.recent", "Recently Played");
    m.insert("library.favorites", "Favorites");
    m.insert("library.playlists", "Playlists");
    m.insert("library.add_to_favorites", "Add to Favorites");
    m.insert("library.remove_from_favorites", "Remove from Favorites");
    m.insert("library.create_playlist", "Create Playlist");
    m.insert("library.delete_playlist", "Delete Playlist");
    m.insert("library.rename", "Rename");
    m.insert("library.delete", "Delete");
    m.insert("library.info", "Info");
    m.insert("library.share", "Share");
    
    // Settings
    m.insert("settings.title", "Settings");
    m.insert("settings.general", "General");
    m.insert("settings.appearance", "Appearance");
    m.insert("settings.playback", "Playback");
    m.insert("settings.subtitles", "Subtitles");
    m.insert("settings.audio", "Audio");
    m.insert("settings.network", "Network");
    m.insert("settings.storage", "Storage");
    m.insert("settings.keyboard", "Keyboard Shortcuts");
    m.insert("settings.about", "About");
    
    // Language settings
    m.insert("settings.language", "Language");
    m.insert("settings.language_description", "Choose your preferred language");
    m.insert("settings.theme", "Theme");
    m.insert("settings.theme_dark", "Dark");
    m.insert("settings.theme_light", "Light");
    m.insert("settings.theme_system", "System");
    
    // Errors
    m.insert("errors.file_not_found", "File not found: {file}");
    m.insert("errors.playback_failed", "Playback failed");
    m.insert("errors.codec_not_supported", "Codec not supported");
    m.insert("errors.network_error", "Network error");
    m.insert("errors.subtitle_not_found", "Subtitles not found");
    m.insert("errors.download_failed", "Download failed");
    m.insert("errors.insufficient_storage", "Insufficient storage");
    m.insert("errors.permission_denied", "Permission denied");
    
    // Marketplace
    m.insert("marketplace.title", "Marketplace");
    m.insert("marketplace.plugins", "Plugins");
    m.insert("marketplace.themes", "Themes");
    m.insert("marketplace.extensions", "Extensions");
    m.insert("marketplace.installed", "Installed");
    m.insert("marketplace.install", "Install");
    m.insert("marketplace.uninstall", "Uninstall");
    m.insert("marketplace.update", "Update");
    m.insert("marketplace.search", "Search marketplace...");
    
    // About
    m.insert("about.title", "About Vantis Media Player");
    m.insert("about.version", "Version");
    m.insert("about.license", "License");
    m.insert("about.copyright", "Copyright");
    m.insert("about.credits", "Credits");
    m.insert("about.licenses", "Open Source Licenses");
    
    m
}

// ============================================================================
// POLISH TRANSLATIONS
// ============================================================================

fn polish_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Odtwórz");
    m.insert("common.pause", "Pauza");
    m.insert("common.stop", "Zatrzymaj");
    m.insert("common.skip", "Pomiń");
    m.insert("common.previous", "Poprzedni");
    m.insert("common.next", "Następny");
    m.insert("common.volume", "Głośność");
    m.insert("common.mute", "Wycisz");
    m.insert("common.unmute", "Włącz dźwięk");
    m.insert("common.fullscreen", "Pełny ekran");
    m.insert("common.exit_fullscreen", "Wyjdź z pełnego ekranu");
    m.insert("common.settings", "Ustawienia");
    m.insert("common.close", "Zamknij");
    m.insert("common.save", "Zapisz");
    m.insert("common.cancel", "Anuluj");
    m.insert("common.confirm", "Potwierdź");
    m.insert("common.delete", "Usuń");
    m.insert("common.edit", "Edytuj");
    m.insert("common.search", "Szukaj");
    m.insert("common.filter", "Filtruj");
    m.insert("common.sort", "Sortuj");
    m.insert("common.refresh", "Odśwież");
    m.insert("common.loading", "Ładowanie...");
    m.insert("common.error", "Błąd");
    m.insert("common.success", "Sukces");
    m.insert("common.warning", "Ostrzeżenie");
    m.insert("common.info", "Informacja");
    
    // Navigation
    m.insert("nav.home", "Strona główna");
    m.insert("nav.library", "Biblioteka");
    m.insert("nav.discover", "Odkryj");
    m.insert("nav.search", "Szukaj");
    m.insert("nav.downloads", "Pobrane");
    m.insert("nav.settings", "Ustawienia");
    m.insert("nav.account", "Konto");
    m.insert("nav.help", "Pomoc");
    m.insert("nav.about", "O programie");
    
    // Media player
    m.insert("player.play", "Odtwórz");
    m.insert("player.pause", "Pauza");
    m.insert("player.stop", "Zatrzymaj");
    m.insert("player.skip_forward", "Przewiń do przodu");
    m.insert("player.skip_backward", "Przewiń do tyłu");
    m.insert("player.speed", "Prędkość odtwarzania");
    m.insert("player.quality", "Jakość");
    m.insert("player.subtitles", "Napisy");
    m.insert("player.audio_track", "Ścieżka audio");
    m.insert("player.subtitle_none", "Brak");
    m.insert("player.subtitle_search", "Szukaj napisów...");
    m.insert("player.subtitle_download", "Pobierz napisy");
    m.insert("player.subtitle_offset", "Przesunięcie napisów");
    m.insert("player.screenshot", "Zrzut ekranu");
    m.insert("player.record", "Nagrywaj");
    m.insert("player.loop", "Powtarzaj");
    m.insert("player.shuffle", "Losuj");
    m.insert("player.picture_in_picture", "Obraz w obrazie");
    
    // Library
    m.insert("library.title", "Biblioteka");
    m.insert("library.empty", "Twoja biblioteka jest pusta");
    m.insert("library.empty_hint", "Dodaj pliki multimedialne, aby rozpocząć oglądanie");
    m.insert("library.recent", "Ostatnio odtwarzane");
    m.insert("library.favorites", "Ulubione");
    m.insert("library.playlists", "Listy odtwarzania");
    m.insert("library.add_to_favorites", "Dodaj do ulubionych");
    m.insert("library.remove_from_favorites", "Usuń z ulubionych");
    m.insert("library.create_playlist", "Utwórz listę odtwarzania");
    m.insert("library.delete_playlist", "Usuń listę odtwarzania");
    m.insert("library.rename", "Zmień nazwę");
    m.insert("library.delete", "Usuń");
    m.insert("library.info", "Informacje");
    m.insert("library.share", "Udostępnij");
    
    // Settings
    m.insert("settings.title", "Ustawienia");
    m.insert("settings.general", "Ogólne");
    m.insert("settings.appearance", "Wygląd");
    m.insert("settings.playback", "Odtwarzanie");
    m.insert("settings.subtitles", "Napisy");
    m.insert("settings.audio", "Audio");
    m.insert("settings.network", "Sieć");
    m.insert("settings.storage", "Pamięć");
    m.insert("settings.keyboard", "Skróty klawiszowe");
    m.insert("settings.about", "O programie");
    
    // Language settings
    m.insert("settings.language", "Język");
    m.insert("settings.language_description", "Wybierz preferowany język");
    m.insert("settings.theme", "Motyw");
    m.insert("settings.theme_dark", "Ciemny");
    m.insert("settings.theme_light", "Jasny");
    m.insert("settings.theme_system", "Systemowy");
    
    // Errors
    m.insert("errors.file_not_found", "Nie znaleziono pliku: {file}");
    m.insert("errors.playback_failed", "Odtwarzanie nie powiodło się");
    m.insert("errors.codec_not_supported", "Kodek nie jest obsługiwany");
    m.insert("errors.network_error", "Błąd sieci");
    m.insert("errors.subtitle_not_found", "Nie znaleziono napisów");
    m.insert("errors.download_failed", "Pobieranie nie powiodło się");
    m.insert("errors.insufficient_storage", "Niewystarczająca ilość pamięci");
    m.insert("errors.permission_denied", "Brak uprawnień");
    
    // Marketplace
    m.insert("marketplace.title", "Sklep");
    m.insert("marketplace.plugins", "Wtyczki");
    m.insert("marketplace.themes", "Motywy");
    m.insert("marketplace.extensions", "Rozszerzenia");
    m.insert("marketplace.installed", "Zainstalowane");
    m.insert("marketplace.install", "Zainstaluj");
    m.insert("marketplace.uninstall", "Odinstaluj");
    m.insert("marketplace.update", "Aktualizuj");
    m.insert("marketplace.search", "Szukaj w sklepie...");
    
    // About
    m.insert("about.title", "O Vantis Media Player");
    m.insert("about.version", "Wersja");
    m.insert("about.license", "Licencja");
    m.insert("about.copyright", "Prawa autorskie");
    m.insert("about.credits", "Podziękowania");
    m.insert("about.licenses", "Licencje open source");
    
    m
}

// ============================================================================
// GERMAN TRANSLATIONS
// ============================================================================

fn german_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Abspielen");
    m.insert("common.pause", "Pause");
    m.insert("common.stop", "Stopp");
    m.insert("common.skip", "Überspringen");
    m.insert("common.previous", "Vorheriger");
    m.insert("common.next", "Nächster");
    m.insert("common.volume", "Lautstärke");
    m.insert("common.mute", "Stumm");
    m.insert("common.unmute", "Ton an");
    m.insert("common.fullscreen", "Vollbild");
    m.insert("common.exit_fullscreen", "Vollbild beenden");
    m.insert("common.settings", "Einstellungen");
    m.insert("common.close", "Schließen");
    m.insert("common.save", "Speichern");
    m.insert("common.cancel", "Abbrechen");
    m.insert("common.confirm", "Bestätigen");
    m.insert("common.delete", "Löschen");
    m.insert("common.edit", "Bearbeiten");
    m.insert("common.search", "Suchen");
    m.insert("common.filter", "Filtern");
    m.insert("common.sort", "Sortieren");
    m.insert("common.refresh", "Aktualisieren");
    m.insert("common.loading", "Laden...");
    m.insert("common.error", "Fehler");
    m.insert("common.success", "Erfolg");
    m.insert("common.warning", "Warnung");
    m.insert("common.info", "Info");
    
    // Navigation
    m.insert("nav.home", "Startseite");
    m.insert("nav.library", "Bibliothek");
    m.insert("nav.discover", "Entdecken");
    m.insert("nav.search", "Suche");
    m.insert("nav.downloads", "Downloads");
    m.insert("nav.settings", "Einstellungen");
    m.insert("nav.account", "Konto");
    m.insert("nav.help", "Hilfe");
    m.insert("nav.about", "Über");
    
    // Media player
    m.insert("player.play", "Abspielen");
    m.insert("player.pause", "Pause");
    m.insert("player.stop", "Stopp");
    m.insert("player.skip_forward", "Vorwärts springen");
    m.insert("player.skip_backward", "Rückwärts springen");
    m.insert("player.speed", "Wiedergabegeschwindigkeit");
    m.insert("player.quality", "Qualität");
    m.insert("player.subtitles", "Untertitel");
    m.insert("player.audio_track", "Audiospur");
    m.insert("player.subtitle_none", "Keine");
    m.insert("player.subtitle_search", "Untertitel suchen...");
    m.insert("player.subtitle_download", "Untertitel herunterladen");
    
    // Library
    m.insert("library.title", "Bibliothek");
    m.insert("library.empty", "Ihre Bibliothek ist leer");
    m.insert("library.empty_hint", "Fügen Sie Mediendateien hinzu, um mit dem Ansehen zu beginnen");
    m.insert("library.recent", "Zuletzt gespielt");
    m.insert("library.favorites", "Favoriten");
    m.insert("library.playlists", "Wiedergabelisten");
    
    // Settings
    m.insert("settings.title", "Einstellungen");
    m.insert("settings.language", "Sprache");
    m.insert("settings.language_description", "Wählen Sie Ihre bevorzugte Sprache");
    m.insert("settings.theme", "Design");
    m.insert("settings.theme_dark", "Dunkel");
    m.insert("settings.theme_light", "Hell");
    m.insert("settings.theme_system", "System");
    
    // Errors
    m.insert("errors.file_not_found", "Datei nicht gefunden: {file}");
    m.insert("errors.playback_failed", "Wiedergabe fehlgeschlagen");
    m.insert("errors.network_error", "Netzwerkfehler");
    
    // Marketplace
    m.insert("marketplace.title", "Marktplatz");
    m.insert("marketplace.plugins", "Plugins");
    m.insert("marketplace.themes", "Designs");
    m.insert("marketplace.extensions", "Erweiterungen");
    
    m
}

// ============================================================================
// CHINESE TRANSLATIONS
// ============================================================================

fn chinese_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "播放");
    m.insert("common.pause", "暂停");
    m.insert("common.stop", "停止");
    m.insert("common.skip", "跳过");
    m.insert("common.previous", "上一个");
    m.insert("common.next", "下一个");
    m.insert("common.volume", "音量");
    m.insert("common.mute", "静音");
    m.insert("common.unmute", "取消静音");
    m.insert("common.fullscreen", "全屏");
    m.insert("common.exit_fullscreen", "退出全屏");
    m.insert("common.settings", "设置");
    m.insert("common.close", "关闭");
    m.insert("common.save", "保存");
    m.insert("common.cancel", "取消");
    m.insert("common.confirm", "确认");
    m.insert("common.delete", "删除");
    m.insert("common.edit", "编辑");
    m.insert("common.search", "搜索");
    m.insert("common.filter", "筛选");
    m.insert("common.sort", "排序");
    m.insert("common.refresh", "刷新");
    m.insert("common.loading", "加载中...");
    m.insert("common.error", "错误");
    m.insert("common.success", "成功");
    m.insert("common.warning", "警告");
    m.insert("common.info", "信息");
    
    // Navigation
    m.insert("nav.home", "主页");
    m.insert("nav.library", "媒体库");
    m.insert("nav.discover", "发现");
    m.insert("nav.search", "搜索");
    m.insert("nav.downloads", "下载");
    m.insert("nav.settings", "设置");
    m.insert("nav.account", "账户");
    m.insert("nav.help", "帮助");
    m.insert("nav.about", "关于");
    
    // Media player
    m.insert("player.play", "播放");
    m.insert("player.pause", "暂停");
    m.insert("player.stop", "停止");
    m.insert("player.skip_forward", "快进");
    m.insert("player.skip_backward", "快退");
    m.insert("player.speed", "播放速度");
    m.insert("player.quality", "画质");
    m.insert("player.subtitles", "字幕");
    m.insert("player.audio_track", "音轨");
    m.insert("player.subtitle_none", "无");
    m.insert("player.subtitle_search", "搜索字幕...");
    m.insert("player.subtitle_download", "下载字幕");
    
    // Library
    m.insert("library.title", "媒体库");
    m.insert("library.empty", "您的媒体库是空的");
    m.insert("library.empty_hint", "添加媒体文件开始观看");
    m.insert("library.recent", "最近播放");
    m.insert("library.favorites", "收藏夹");
    m.insert("library.playlists", "播放列表");
    
    // Settings
    m.insert("settings.title", "设置");
    m.insert("settings.language", "语言");
    m.insert("settings.language_description", "选择您的首选语言");
    m.insert("settings.theme", "主题");
    m.insert("settings.theme_dark", "深色");
    m.insert("settings.theme_light", "浅色");
    m.insert("settings.theme_system", "系统");
    
    // Errors
    m.insert("errors.file_not_found", "找不到文件: {file}");
    m.insert("errors.playback_failed", "播放失败");
    m.insert("errors.network_error", "网络错误");
    
    // Marketplace
    m.insert("marketplace.title", "市场");
    m.insert("marketplace.plugins", "插件");
    m.insert("marketplace.themes", "主题");
    m.insert("marketplace.extensions", "扩展");
    
    m
}

// ============================================================================
// RUSSIAN TRANSLATIONS
// ============================================================================

fn russian_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Воспроизвести");
    m.insert("common.pause", "Пауза");
    m.insert("common.stop", "Стоп");
    m.insert("common.skip", "Пропустить");
    m.insert("common.previous", "Предыдущий");
    m.insert("common.next", "Следующий");
    m.insert("common.volume", "Громкость");
    m.insert("common.mute", "Отключить звук");
    m.insert("common.unmute", "Включить звук");
    m.insert("common.fullscreen", "Полный экран");
    m.insert("common.exit_fullscreen", "Выйти из полного экрана");
    m.insert("common.settings", "Настройки");
    m.insert("common.close", "Закрыть");
    m.insert("common.save", "Сохранить");
    m.insert("common.cancel", "Отмена");
    m.insert("common.confirm", "Подтвердить");
    m.insert("common.delete", "Удалить");
    m.insert("common.edit", "Редактировать");
    m.insert("common.search", "Поиск");
    m.insert("common.filter", "Фильтр");
    m.insert("common.sort", "Сортировка");
    m.insert("common.refresh", "Обновить");
    m.insert("common.loading", "Загрузка...");
    m.insert("common.error", "Ошибка");
    m.insert("common.success", "Успешно");
    m.insert("common.warning", "Предупреждение");
    m.insert("common.info", "Информация");
    
    // Navigation
    m.insert("nav.home", "Главная");
    m.insert("nav.library", "Библиотека");
    m.insert("nav.discover", "Обзор");
    m.insert("nav.search", "Поиск");
    m.insert("nav.downloads", "Загрузки");
    m.insert("nav.settings", "Настройки");
    m.insert("nav.account", "Аккаунт");
    m.insert("nav.help", "Помощь");
    m.insert("nav.about", "О программе");
    
    // Media player
    m.insert("player.play", "Воспроизвести");
    m.insert("player.pause", "Пауза");
    m.insert("player.stop", "Стоп");
    m.insert("player.skip_forward", "Вперёд");
    m.insert("player.skip_backward", "Назад");
    m.insert("player.speed", "Скорость воспроизведения");
    m.insert("player.quality", "Качество");
    m.insert("player.subtitles", "Субтитры");
    m.insert("player.audio_track", "Аудиодорожка");
    m.insert("player.subtitle_none", "Нет");
    m.insert("player.subtitle_search", "Поиск субтитров...");
    m.insert("player.subtitle_download", "Скачать субтитры");
    
    // Library
    m.insert("library.title", "Библиотека");
    m.insert("library.empty", "Ваша библиотека пуста");
    m.insert("library.empty_hint", "Добавьте медиафайлы для начала просмотра");
    m.insert("library.recent", "Недавние");
    m.insert("library.favorites", "Избранное");
    m.insert("library.playlists", "Плейлисты");
    
    // Settings
    m.insert("settings.title", "Настройки");
    m.insert("settings.language", "Язык");
    m.insert("settings.language_description", "Выберите предпочтительный язык");
    m.insert("settings.theme", "Тема");
    m.insert("settings.theme_dark", "Тёмная");
    m.insert("settings.theme_light", "Светлая");
    m.insert("settings.theme_system", "Системная");
    
    // Errors
    m.insert("errors.file_not_found", "Файл не найден: {file}");
    m.insert("errors.playback_failed", "Ошибка воспроизведения");
    m.insert("errors.network_error", "Ошибка сети");
    
    // Marketplace
    m.insert("marketplace.title", "Маркетплейс");
    m.insert("marketplace.plugins", "Плагины");
    m.insert("marketplace.themes", "Темы");
    m.insert("marketplace.extensions", "Расширения");
    
    m
}

// ============================================================================
// KOREAN TRANSLATIONS
// ============================================================================

fn korean_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "재생");
    m.insert("common.pause", "일시정지");
    m.insert("common.stop", "정지");
    m.insert("common.skip", "건너뛰기");
    m.insert("common.previous", "이전");
    m.insert("common.next", "다음");
    m.insert("common.volume", "볼륨");
    m.insert("common.mute", "음소거");
    m.insert("common.unmute", "음소거 해제");
    m.insert("common.fullscreen", "전체 화면");
    m.insert("common.exit_fullscreen", "전체 화면 종료");
    m.insert("common.settings", "설정");
    m.insert("common.close", "닫기");
    m.insert("common.save", "저장");
    m.insert("common.cancel", "취소");
    m.insert("common.confirm", "확인");
    m.insert("common.delete", "삭제");
    m.insert("common.edit", "편집");
    m.insert("common.search", "검색");
    m.insert("common.filter", "필터");
    m.insert("common.sort", "정렬");
    m.insert("common.refresh", "새로고침");
    m.insert("common.loading", "로딩 중...");
    m.insert("common.error", "오류");
    m.insert("common.success", "성공");
    m.insert("common.warning", "경고");
    m.insert("common.info", "정보");
    
    // Navigation
    m.insert("nav.home", "홈");
    m.insert("nav.library", "라이브러리");
    m.insert("nav.discover", "검색");
    m.insert("nav.search", "검색");
    m.insert("nav.downloads", "다운로드");
    m.insert("nav.settings", "설정");
    m.insert("nav.account", "계정");
    m.insert("nav.help", "도움말");
    m.insert("nav.about", "정보");
    
    // Media player
    m.insert("player.play", "재생");
    m.insert("player.pause", "일시정지");
    m.insert("player.stop", "정지");
    m.insert("player.skip_forward", "앞으로 건너뛰기");
    m.insert("player.skip_backward", "뒤로 건너뛰기");
    m.insert("player.speed", "재생 속도");
    m.insert("player.quality", "화질");
    m.insert("player.subtitles", "자막");
    m.insert("player.audio_track", "오디오 트랙");
    m.insert("player.subtitle_none", "없음");
    m.insert("player.subtitle_search", "자막 검색...");
    m.insert("player.subtitle_download", "자막 다운로드");
    
    // Library
    m.insert("library.title", "라이브러리");
    m.insert("library.empty", "라이브러리가 비어 있습니다");
    m.insert("library.empty_hint", "미디어 파일을 추가하여 시청을 시작하세요");
    m.insert("library.recent", "최근 재생");
    m.insert("library.favorites", "즐겨찾기");
    m.insert("library.playlists", "재생목록");
    
    // Settings
    m.insert("settings.title", "설정");
    m.insert("settings.language", "언어");
    m.insert("settings.language_description", "선호하는 언어를 선택하세요");
    m.insert("settings.theme", "테마");
    m.insert("settings.theme_dark", "다크");
    m.insert("settings.theme_light", "라이트");
    m.insert("settings.theme_system", "시스템");
    
    // Errors
    m.insert("errors.file_not_found", "파일을 찾을 수 없습니다: {file}");
    m.insert("errors.playback_failed", "재생 실패");
    m.insert("errors.network_error", "네트워크 오류");
    
    // Marketplace
    m.insert("marketplace.title", "마켓플레이스");
    m.insert("marketplace.plugins", "플러그인");
    m.insert("marketplace.themes", "테마");
    m.insert("marketplace.extensions", "확장 프로그램");
    
    m
}

// ============================================================================
// SPANISH TRANSLATIONS
// ============================================================================

fn spanish_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Reproducir");
    m.insert("common.pause", "Pausar");
    m.insert("common.stop", "Detener");
    m.insert("common.skip", "Saltar");
    m.insert("common.previous", "Anterior");
    m.insert("common.next", "Siguiente");
    m.insert("common.volume", "Volumen");
    m.insert("common.mute", "Silenciar");
    m.insert("common.unmute", "Activar sonido");
    m.insert("common.fullscreen", "Pantalla completa");
    m.insert("common.exit_fullscreen", "Salir de pantalla completa");
    m.insert("common.settings", "Configuración");
    m.insert("common.close", "Cerrar");
    m.insert("common.save", "Guardar");
    m.insert("common.cancel", "Cancelar");
    m.insert("common.confirm", "Confirmar");
    m.insert("common.delete", "Eliminar");
    m.insert("common.edit", "Editar");
    m.insert("common.search", "Buscar");
    m.insert("common.filter", "Filtrar");
    m.insert("common.sort", "Ordenar");
    m.insert("common.refresh", "Actualizar");
    m.insert("common.loading", "Cargando...");
    m.insert("common.error", "Error");
    m.insert("common.success", "Éxito");
    m.insert("common.warning", "Advertencia");
    m.insert("common.info", "Información");
    
    // Navigation
    m.insert("nav.home", "Inicio");
    m.insert("nav.library", "Biblioteca");
    m.insert("nav.discover", "Descubrir");
    m.insert("nav.search", "Buscar");
    m.insert("nav.downloads", "Descargas");
    m.insert("nav.settings", "Configuración");
    m.insert("nav.account", "Cuenta");
    m.insert("nav.help", "Ayuda");
    m.insert("nav.about", "Acerca de");
    
    // Media player
    m.insert("player.play", "Reproducir");
    m.insert("player.pause", "Pausar");
    m.insert("player.stop", "Detener");
    m.insert("player.skip_forward", "Adelantar");
    m.insert("player.skip_backward", "Retroceder");
    m.insert("player.speed", "Velocidad de reproducción");
    m.insert("player.quality", "Calidad");
    m.insert("player.subtitles", "Subtítulos");
    m.insert("player.audio_track", "Pista de audio");
    m.insert("player.subtitle_none", "Ninguno");
    m.insert("player.subtitle_search", "Buscar subtítulos...");
    m.insert("player.subtitle_download", "Descargar subtítulos");
    
    // Library
    m.insert("library.title", "Biblioteca");
    m.insert("library.empty", "Tu biblioteca está vacía");
    m.insert("library.empty_hint", "Añade archivos multimedia para empezar a ver");
    m.insert("library.recent", "Reproducido recientemente");
    m.insert("library.favorites", "Favoritos");
    m.insert("library.playlists", "Listas de reproducción");
    
    // Settings
    m.insert("settings.title", "Configuración");
    m.insert("settings.language", "Idioma");
    m.insert("settings.language_description", "Elige tu idioma preferido");
    m.insert("settings.theme", "Tema");
    m.insert("settings.theme_dark", "Oscuro");
    m.insert("settings.theme_light", "Claro");
    m.insert("settings.theme_system", "Sistema");
    
    // Errors
    m.insert("errors.file_not_found", "Archivo no encontrado: {file}");
    m.insert("errors.playback_failed", "Error de reproducción");
    m.insert("errors.network_error", "Error de red");
    
    // Marketplace
    m.insert("marketplace.title", "Mercado");
    m.insert("marketplace.plugins", "Plugins");
    m.insert("marketplace.themes", "Temas");
    m.insert("marketplace.extensions", "Extensiones");
    
    m
}

// ============================================================================
// FRENCH TRANSLATIONS
// ============================================================================

fn french_translations() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Common UI elements
    m.insert("common.play", "Lecture");
    m.insert("common.pause", "Pause");
    m.insert("common.stop", "Arrêt");
    m.insert("common.skip", "Passer");
    m.insert("common.previous", "Précédent");
    m.insert("common.next", "Suivant");
    m.insert("common.volume", "Volume");
    m.insert("common.mute", "Muet");
    m.insert("common.unmute", "Activer le son");
    m.insert("common.fullscreen", "Plein écran");
    m.insert("common.exit_fullscreen", "Quitter le plein écran");
    m.insert("common.settings", "Paramètres");
    m.insert("common.close", "Fermer");
    m.insert("common.save", "Enregistrer");
    m.insert("common.cancel", "Annuler");
    m.insert("common.confirm", "Confirmer");
    m.insert("common.delete", "Supprimer");
    m.insert("common.edit", "Modifier");
    m.insert("common.search", "Rechercher");
    m.insert("common.filter", "Filtrer");
    m.insert("common.sort", "Trier");
    m.insert("common.refresh", "Actualiser");
    m.insert("common.loading", "Chargement...");
    m.insert("common.error", "Erreur");
    m.insert("common.success", "Succès");
    m.insert("common.warning", "Avertissement");
    m.insert("common.info", "Information");
    
    // Navigation
    m.insert("nav.home", "Accueil");
    m.insert("nav.library", "Bibliothèque");
    m.insert("nav.discover", "Découvrir");
    m.insert("nav.search", "Recherche");
    m.insert("nav.downloads", "Téléchargements");
    m.insert("nav.settings", "Paramètres");
    m.insert("nav.account", "Compte");
    m.insert("nav.help", "Aide");
    m.insert("nav.about", "À propos");
    
    // Media player
    m.insert("player.play", "Lecture");
    m.insert("player.pause", "Pause");
    m.insert("player.stop", "Arrêt");
    m.insert("player.skip_forward", "Avancer");
    m.insert("player.skip_backward", "Reculer");
    m.insert("player.speed", "Vitesse de lecture");
    m.insert("player.quality", "Qualité");
    m.insert("player.subtitles", "Sous-titres");
    m.insert("player.audio_track", "Piste audio");
    m.insert("player.subtitle_none", "Aucun");
    m.insert("player.subtitle_search", "Rechercher des sous-titres...");
    m.insert("player.subtitle_download", "Télécharger les sous-titres");
    
    // Library
    m.insert("library.title", "Bibliothèque");
    m.insert("library.empty", "Votre bibliothèque est vide");
    m.insert("library.empty_hint", "Ajoutez des fichiers multimédias pour commencer à regarder");
    m.insert("library.recent", "Récemment lus");
    m.insert("library.favorites", "Favoris");
    m.insert("library.playlists", "Listes de lecture");
    
    // Settings
    m.insert("settings.title", "Paramètres");
    m.insert("settings.language", "Langue");
    m.insert("settings.language_description", "Choisissez votre langue préférée");
    m.insert("settings.theme", "Thème");
    m.insert("settings.theme_dark", "Sombre");
    m.insert("settings.theme_light", "Clair");
    m.insert("settings.theme_system", "Système");
    
    // Errors
    m.insert("errors.file_not_found", "Fichier introuvable : {file}");
    m.insert("errors.playback_failed", "Échec de la lecture");
    m.insert("errors.network_error", "Erreur réseau");
    
    // Marketplace
    m.insert("marketplace.title", "Marketplace");
    m.insert("marketplace.plugins", "Extensions");
    m.insert("marketplace.themes", "Thèmes");
    m.insert("marketplace.extensions", "Modules");
    
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_english_translation() {
        let translator = Translator::new(Language::English);
        assert_eq!(translator.translate("common.play"), "Play");
        assert_eq!(translator.translate("nav.home"), "Home");
    }
    
    #[test]
    fn test_polish_translation() {
        let translator = Translator::new(Language::Polish);
        assert_eq!(translator.translate("common.play"), "Odtwórz");
        assert_eq!(translator.translate("nav.home"), "Strona główna");
    }
    
    #[test]
    fn test_fallback_to_english() {
        // If a key is missing in Polish, fall back to English
        let translator = Translator::new(Language::Polish);
        // These keys should exist in both
        assert!(!translator.translate("common.play").starts_with("⚠️"));
    }
    
    #[test]
    fn test_missing_key() {
        let translator = Translator::new(Language::English);
        let result = translator.translate("nonexistent.key");
        assert!(result.starts_with("⚠️"));
    }
    
    #[test]
    fn test_has_key() {
        let translator = Translator::new(Language::English);
        assert!(translator.has("common.play"));
        assert!(!translator.has("nonexistent.key"));
    }
}