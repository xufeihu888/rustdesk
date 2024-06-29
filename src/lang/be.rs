lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Статус"),
        ("Your Desktop", "Ваш працоўны стол"),
        ("desk_tip", "Ваш працоўны стол даступны з гэтым ID і паролем."),
        ("Password", "Пароль"),
        ("Ready", "Гатовы"),
        ("Established", "Усталявана"),
        ("connecting_status", "Падключэнне да сеткі RustDesk..."),
        ("Enable service", "Уключыць службу"),
        ("Start service", "Запусціць службу"),
        ("Service is running", "Служба запушчана"),
        ("Service is not running", "Служба не запушчана"),
        ("not_ready_status", "Не падключана. Праверце злучэнне."),
        ("Control Remote Desktop", "Кіраванне выдаленым працоўным сталом"),
        ("Transfer file", "Перадаць файлы"),
        ("Connect", "Падключыцца"),
        ("Recent sessions", "Апошнія сеансы"),
        ("Address book", "Адрасная кніга"),
        ("Confirmation", "Пацвярджэнне"),
        ("TCP tunneling", "TCP-тунэляванне"),
        ("Remove", "Выдаліць"),
        ("Refresh random password", "Абнавіць выпадковы пароль"),
        ("Set your own password", "Усталяваць свой пароль"),
        ("Enable keyboard/mouse", "Выкарыстоўваць клавіятуру/мыш"),
        ("Enable clipboard", "Выкарыстоўваць буфер абмену"),
        ("Enable file transfer", "Выкарыстоўваць перадачу файлаў"),
        ("Enable TCP tunneling", "Выкарыстоўваць тунэляванне TCP"),
        ("IP Whitelisting", "Спіс дазволеных IP-адрасоў"),
        ("ID/Relay Server", "ID/Рэтранслятар"),
        ("Import server config", "Імпартаваць канфігурацыю сервера"),
        ("Export Server Config", "Экспартаваць канфігурацыю сервера"),
        ("Import server configuration successfully", "Канфігурацыя сервера паспяхова імпартавана"),
        ("Export server configuration successfully", "Канфігурацыя сервера паспяхова экспартавана"),
        ("Invalid server configuration", "Няправільная канфігурацыя сервера"),
        ("Clipboard is empty", "Буфер абмену пусты"),
        ("Stop service", "Спыніць службу"),
        ("Change ID", "Змяніць ID"),
        ("Your new ID", "Новы ID"),
        ("length %min% to %max%", "даўжыня %min%...%max%"),
        ("starts with a letter", "пачынаецца з літары"),
        ("allowed characters", "дазволеныя сімвалы"),
        ("id_change_tip", "Дапускаюцца толькі сімвалы a-z, A-Z, 0-9 і _ (падкрэсліванне). Першай павінна быць літара a-z, A-Z. Даўжыня ад 6 да 16."),
        ("Website", "Сайт"),
        ("About", "Пра праграму"),
        ("Slogan_tip", "Зроблена з душой у гэтым вар'яцкім свеце!"),
        ("Privacy Statement", "Заява аб канфідэнцыяльнасці"),
        ("Mute", "Адключыць гук"),
        ("Build Date", "Дата зборкі"),
        ("Version", "Версія"),
        ("Home", "Галоўная"),
        ("Audio Input", "Аўдыёўваход"),
        ("Enhancements", "Палепшанні"),
        ("Hardware Codec", "Апаратны кодэк"),
        ("Adaptive bitrate", "Адаптыўны бітрэйт"),
        ("ID Server", "Сервер ID"),
        ("Relay Server", "Рэтранслятар"),
        ("API Server", "Сервер API"),
        ("invalid_http", "Адрас павінен пачынацца з http:// або https://"),
        ("Invalid IP", "Няправільны IP-адрас"),
        ("Invalid format", "Няправільны фармат"),
        ("server_not_support", "Пакуль не падтрымліваецца серверам"),
        ("Not available", "Недаступна"),
        ("Too frequent", "Занадта часта"),
        ("Cancel", "Адмяніць"),
        ("Skip", "Прапусціць"),
        ("Close", "Закрыць"),
        ("Retry", "Паўтор"),
        ("OK", "ОК"),
        ("Password Required", "Патрабуецца пароль"),
        ("Please enter your password", "Увядзіце пароль"),
        ("Remember password", "Запомніць пароль"),
        ("Wrong Password", "Няправільны пароль"),
        ("Do you want to enter again?", "Паўтарыць уваход?"),
        ("Connection Error", "Памылка падключэння"),
        ("Error", "Памылка"),
        ("Reset by the peer", "Скінута выдаленым вузлом"),
        ("Connecting...", "Падключэнне..."),
        ("Connection in progress. Please wait.", "Выконваецца падключэнне. Пачакайце."),
        ("Please try 1 minute later", "Паспрабуйце праз хвіліну"),
        ("Login Error", "Памылка ўваходу"),
        ("Successful", "Паспяхова"),
        ("Connected, waiting for image...", "Падключана, чаканне выявы..."),
        ("Name", "Імя"),
        ("Type", "Тып"),
        ("Modified", "Зменена"),
        ("Size", "Памер"),
        ("Show Hidden Files", "Паказаць схаваныя файлы"),
        ("Receive", "Атрымаць"),
        ("Send", "Адправіць"),
        ("Refresh File", "Абнавіць файл"),
        ("Local", "Лакальны"),
        ("Remote", "Выдалены"),
        ("Remote Computer", "Выдалены камп'ютар"),
        ("Local Computer", "Лакальны камп'ютар"),
        ("Confirm Delete", "Пацвердзіць выдаленне"),
        ("Delete", "Выдаліць"),
        ("Properties", "Уласцівасці"),
        ("Multi Select", "Шматлікі выбар"),
        ("Select All", "Абраць усе"),
        ("Unselect All", "Зняць усе"),
        ("Empty Directory", "Пустая тэчка"),
        ("Not an empty directory", "Тэчка не пустая"),
        ("Are you sure you want to delete this file?", "Выдаліць гэты файл?"),
        ("Are you sure you want to delete this empty directory?", "Выдаліць пустую тэчку?"),
        ("Are you sure you want to delete the file of this directory?", "Выдаліць файл з гэтай тэчкі?"),
        ("Do this for all conflicts", "Прымяніць да ўсіх канфліктаў"),
        ("This is irreversible!", "Гэта неабаротна!"),
        ("Deleting", "Выдаленне"),
        ("files", "файлы"),
        ("Waiting", "Чаканне"),
        ("Finished", "Завершана"),
        ("Speed", "Хуткасць"),
        ("Custom Image Quality", "Якасць выявы па запыце"),
        ("Privacy mode", "Рэжым прыватнасці"),
        ("Block user input", "Забараніць ўвод на аддаленай прыладзе"),
        ("Unblock user input", "Адблакіраваць ўвод на аддаленай прыладзе"),
        ("Adjust Window", "Наладзіць акно"),
        ("Original", "Арыгінал"),
        ("Shrink", "Сціснуць"),
        ("Stretch", "Расцягнуць"),
        ("Scrollbar", "Паласа пракруткі"),
        ("ScrollAuto", "Аўта-пракрутка"),
        ("Good image quality", "Добрая якасць выявы"),
        ("Balanced", "Баланс паміж якасцю і адказам"),
        ("Optimize reaction time", "Оптымізацыя часу адказу"),
        ("Custom", "Зададзена карыстальнікам"),
        ("Show remote cursor", "Паказваць аддалены курсор"),
        ("Show quality monitor", "Паказваць манітор якасці"),
        ("Disable clipboard", "Адключыць буфер абмену"),
        ("Lock after session end", "Заблакаваць уліковы запіс пасля сеансу"),
        ("Insert", "Уставіць"),
        ("Insert Lock", "Заблакаваць уліковы запіс"),
        ("Refresh", "Абнавіць"),
        ("ID does not exist", "ID не існуе"),
        ("Failed to connect to rendezvous server", "Немагчыма падключыцца да паседкавага сервера"),
        ("Please try later", "Паспрабуйце пазней"),
        ("Remote desktop is offline", "Аддаленая прылада не ў сетцы"),
        ("Key mismatch", "Неадпаведнасць ключоў"),
        ("Timeout", "Час чакання скончыўся"),
        ("Failed to connect to relay server", "Немагчыма падключыцца да рэтранслятара"),
        ("Failed to connect via rendezvous server", "Немагчыма падключыцца праз паседкавы сервер"),
        ("Failed to connect via relay server", "Немагчыма падключыцца праз рэтранслятар"),
        ("Failed to make direct connection to remote desktop", "Не ўдалося ўсталяваць прамое падключэнне да аддаленага працоўнага стала"),
        ("Set Password", "Усталяваць пароль"),
        ("OS Password", "Пароль ўваходу ў аперацыйную сістэму"),
        ("install_tip", "У некаторых выпадках RustDesk можа працаваць няправільна на аддаленым вузле з-за UAC. Каб пазбегнуць магчымых праблем з UAC, націсніце кнопку ніжэй для ўстаноўкі RustDesk у сістэме."),
        ("Click to upgrade", "Абнавіць"),
        ("Click to download", "Спампаваць"),
        ("Click to update", "Абнавіць"),
        ("Configure", "Наладзіць"),
        ("config_acc", "Каб аддаленна кіраваць сваім працоўным сталом, вам неабходна дазволіць RustDesk правы доступу."),
        ("config_screen", "Для аддаленага доступу да працоўнага сталу вам неабходна дазволіць RustDesk правы здымку экрана."),
        ("Installing ...", "Ідзе ўстаноўка..."),
        ("Install", "Усталяваць"),
        ("Installation", "Устаноўка"),
        ("Installation Path", "Шлях устаноўкі"),
        ("Create start menu shortcuts", "Стварыць ярлыкі ў меню \"Пуск\""),
        ("Create desktop icon", "Стварыць значок на працоўным стале"),
        ("agreement_tip", "Пачынаючы ўстаноўку, вы прымаеце ўмовы ліцэнзійнага ўгоды."),
        ("Accept and Install", "Прыняць і ўсталяваць"),
        ("End-user license agreement", "Ліцэнзійная ўгода з канчатковым карыстальнікам"),
        ("Generating ...", "Генеруецца..."),
        ("Your installation is lower version.", "Ваша ўстаноўка ніжэйшай версіі"),
        ("not_close_tcp_tip", "Не зачыняць гэта акно пры выкарыстанні тунэлю."),
        ("Listening ...", "Праслухоўванне..."),
        ("Remote Host", "Аддалены хост"),
        ("Remote Port", "Аддалены порт"),
        ("Action", "Дзеянне"),
        ("Add", "Дадаць"),
        ("Local Port", "Лакальны порт"),
        ("Local Address", "Лакальны адрас"),
        ("Change Local Port", "Змяніць лакальны порт"),
        ("setup_server_tip", "Для хуткага падключэння наладзьце свой сервер."),
        ("Too short, at least 6 characters.", "Занадта кароткі, мінімум 6 сімвалаў."),
        ("The confirmation is not identical.", "Пацверджанне не супадае."),
        ("Permissions", "Дазволы"),
        ("Accept", "Прыняць"),
        ("Dismiss", "Адхіліць"),
        ("Disconnect", "Адключыць"),
        ("Enable file copy and paste", "Дазволіць капіраванне і ўстаўку файлаў"),
        ("Connected", "Падключана"),
        ("Direct and encrypted connection", "Прамое і зашыфраванае падключэнне"),
        ("Relayed and encrypted connection", "Рэтрансляванае і зашыфраванае падключэнне"),
        ("Direct and unencrypted connection", "Прамое і незашыфраванае падключэнне"),
        ("Relayed and unencrypted connection", "Рэтрансляванае і незашыфраванае падключэнне"),
        ("Enter Remote ID", "Увядзіце дыстанцыйны ID"),
        ("Enter your password", "Увядзіце пароль"),
        ("Logging in...", "Уваход..."),
        ("Enable RDP session sharing", "Дазволіць абмен сеансамі RDP"),
        ("Auto Login", "Аўтаматычны ўваход у ўліковы запіс"),
        ("Enable direct IP access", "Дазволіць прамы доступ па IP-адрасе"),
        ("Rename", "Перайменаваць"),
        ("Space", "Месца"),
        ("Create desktop shortcut", "Стварыць ярлык на працоўным стале"),
        ("Change Path", "Змяніць шлях"),
        ("Create Folder", "Стварыць тэчку"),
        ("Please enter the folder name", "Калі ласка, увядзіце імя тэчкі"),
        ("Fix it", "Выправіць"),
        ("Warning", "Папярэджанне"),
        ("Login screen using Wayland is not supported", "Уваход у сістэму з выкарыстаннем Wayland не падтрымліваецца"),
        ("Reboot required", "Патрабуецца перазагрузка"),
        ("Unsupported display server", "Непадтрымліваемы сервер адлюстравання"),
        ("x11 expected", "Чакаецца X11"),
        ("Port", "Порт"),
        ("Settings", "Налады"),
        ("Username", "Імя карыстальніка"),
        ("Invalid port", "Няправільны порт"),
        ("Closed manually by the peer", "Зачынена аддаленым вузлом уручную"),
        ("Enable remote configuration modification", "Дазволіць змену канфігурацыі аддалена"),
        ("Run without install", "Запусціць без ўстаноўкі"),
        ("Connect via relay", "Падключыцца праз рэтранслятар"),
        ("Always connect via relay", "Заўсёды падключацца праз рэтранслятар"),
        ("whitelist_tip", "Толькі IP-адрэсы з белага спісу могуць атрымаць доступ да маёй прылады."),
        ("Login", "Увайсці"),
        ("Verify", "Праверыць"),
        ("Remember me", "Запомніць мяне"),
        ("Trust this device", "Даверыць гэтую прыладу"),
        ("Verification code", "Праверачны код"),
        ("verification_tip", "Выяўлена новая прылада, на зарэгістраваны адрас электроннай пошты адпраўлены праверачны код. Увядзіце яго, каб працягнуць уваход у сістэму."),
        ("Logout", "Выйсці"),
        ("Tags", "Тэгі"),
        ("Search ID", "Пошук по ID"),
        ("whitelist_sep", "Аддзяліць запятой, коскай з запятой, прабелам ці новым радком."),
        ("Add ID", "Дадаць ID"),
        ("Add Tag", "Дадаць тэг"),
        ("Unselect all tags", "Скасаваць выбар усіх тэгаў"),
        ("Network error", "Памылка сеткі"),
        ("Username missed", "Адсутнічае імя карыстальніка"),
        ("Password missed", "Забыты пароль"),
        ("Wrong credentials", "Няправільныя імя ці пароль"),
        ("The verification code is incorrect or has expired", "Праверачны код няправільны або скончыўся тэрмін яго дзеяння"),
        ("Edit Tag", "Рэдагаваць тэг"),
        ("Forget Password", "Забыць пароль"),
        ("Favorites", "Абранае"),
        ("Add to Favorites", "Дадаць у абранае"),
        ("Remove from Favorites", "Выдаліць з абранага"),
        ("Empty", "Пуста"),
        ("Invalid folder name", "Недапушчальнае імя тэчкі"),
        ("Socks5 Proxy", "Socks5-проксі"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s)-проксі"),
        ("Discovered", "Знойдзена"),
        ("install_daemon_tip", "Для запуску пры загрузцы неабходна ўстанавіць сістэмную службу"),
        ("Remote ID", "Аддалены ID"),
        ("Paste", "Уставіць"),
        ("Paste here?", "Уставіць тут?"),
        ("Are you sure to close the connection?", "Ці ўпэўненыя, што жадаеце закрыць падключэнне?"),
        ("Download new version", "Спампаваць новую версію"),
        ("Touch mode", "Рэжым сэнсарнага экрана"),
        ("Mouse mode", "Рэжым мышы/трэкпада"),
        ("One-Finger Tap", "Націск адным пальцам"),
        ("Left Mouse", "Левая кнопка мышы"),
        ("One-Long Tap", "Доўгі націск адным пальцам"),
        ("Two-Finger Tap", "Націск двума пальцамі"),
        ("Right Mouse", "Правая кнопка мышы"),
        ("One-Finger Move", "Перамяшчэнне адным пальцам"),
        ("Double Tap & Move", "Двайны націск і перамяшчэнне"),
        ("Mouse Drag", "Перацягванне мышшу"),
        ("Three-Finger vertically", "Трыма пальцамі па вертыкалі"),
        ("Mouse Wheel", "Кола мышы"),
        ("Two-Finger Move", "Перамяшчэнне двума пальцамі"),
        ("Canvas Move", "Перамяшчэнне палатна"),
        ("Pinch to Zoom", "Маштабаванне сціскам"),
        ("Canvas Zoom", "Маштаб палатна"),
        ("Reset canvas", "Скінуць палатно"),
        ("No permission of file transfer", "Няма дазволу на перадачу файлаў"),
        ("Note", "Нататка"),
        ("Connection", "Падключэнне"),
        ("Share Screen", "Дзяліцца экранам"),
        ("Chat", "Чат"),
        ("Total", "Усяго"),
        ("items", "элементы"),
        ("Selected", "Выбрана"),
        ("Screen Capture", "Захоп экрана"),
        ("Input Control", "Кіраванне ўводам"),
        ("Audio Capture", "Захоп аўдыё"),
        ("File Connection", "Падлучэнне перадачы файлаў"),
        ("Screen Connection", "Падлучэнне прагляду/кіравання экранам"),
        ("Do you accept?", "Ці вы згодны?"),
        ("Open System Setting", "Адкрыць налады сістэмы"),
        ("How to get Android input permission?", "Як атрымаць дазвол на ўвод Android?"),
        ("android_input_permission_tip1", "Каб аддалёная прылада магла кіраваць вашай Android-прыладай з дапамогай мышы або націсканняў, неабходна дазволіць RustDesk выкарыстоўваць паслугу \"Асаблівыя магчымасці\"."),
        ("android_input_permission_tip2", "Зайдзіце на адпаведную старонку сістэмных налад, знайдзіце і ўступіце ў \"Устаноўленыя паслугі\", уключыце паслугу \"RustDesk Input\"."),
        ("android_new_connection_tip", "Атрыманы запыт на кіраванне вашай бягучай прыладай."),
        ("android_service_will_start_tip", "Уключэнне захопу экрана аўтаматычна запускае службу, дазваляючы іншым прыладам запытаць падлучэнне да гэтай прылады."),
        ("android_stop_service_tip", "Закрыццё службы аўтаматычна зачыніць усе ўстаноўленыя падлучэнні."),
        ("android_version_audio_tip", "Бягучая версія Android не падтрымлівае захоп звуку, абнавіце яе да Android 10 ці вышэй."),
        ("android_start_service_tip", "Націсніце [Запусціць службу] або дазвольце [Захоп экрана], каб запусціць службу дэманстрацыі экрана."),
        ("android_permission_may_not_change_tip", "Дазволы для ўстаноўленых падлучэнняў не могуць быць змененыя, неабходна перападключэнне."),
        ("Account", "Уліковы запіс"),
        ("Overwrite", "Перазапісаць"),
        ("This file exists, skip or overwrite this file?", "Файл існуе, прапусціць ці перазапісаць яго?"),
        ("Quit", "Выйсці"),
        ("Help", "Дапамога"),
        ("Failed", "Не ўдалося"),
        ("Succeeded", "Выканана"),
        ("Someone turns on privacy mode, exit", "Хтосьці ўключыў рэжым прыватнасці, выхад"),
        ("Unsupported", "Не падтрымліваецца"),
        ("Peer denied", "Адмоўлена аддаленым вузлом"),
        ("Please install plugins", "Усталюйце плагіны"),
        ("Peer exit", "Аддалены вузел адключаны"),
        ("Failed to turn off", "Немагчыма адключыць"),
        ("Turned off", "Адключаны"),
        ("Language", "Мова"),
        ("Keep RustDesk background service", "Захаваць фонавую службу RustDesk"),
        ("Ignore Battery Optimizations", "Ігнараваць аптымізацыю патрэблення батарэі"),
        ("android_open_battery_optimizations_tip", "Перайдзіце на наступную старонку налад"),
        ("Start on boot", "Запускаць пры загрузцы"),
        ("Start the screen sharing service on boot, requires special permissions", "Запускаць службу дэманстрацыі экрана пры загрузцы (патрабуюцца спецыяльныя дазволы)"),
        ("Connection not allowed", "Падключэнне не дазволена"),
        ("Legacy mode", "Стары рэжым"),
        ("Map mode", "Рэжым супастаўлення"),
        ("Translate mode", "Рэжым перакладу"),
        ("Use permanent password", "Выкарыстоўваць пастаянны пароль"),
        ("Use both passwords", "Выкарыстоўваць абодва паролі"),
        ("Set permanent password", "Устанавіць пастаянны пароль"),
        ("Enable remote restart", "Дазволіць аддалены перазапуск"),
        ("Restart remote device", "Перазапусціць аддаленую прыладу"),
        ("Are you sure you want to restart", "Вы ўпэўненыя, што хочаце перазагрузіць?"),
        ("Restarting remote device", "Перазапуск аддаленай прылады"),
        ("remote_restarting_tip", "Аддаленая прылада перазапускаецца. Закрыйце гэтае паведамленне і праз некаторы час перападключыцеся, выкарыстоўваючы пастаянны пароль."),
        ("Copied", "Скапіравана"),
        ("Exit Fullscreen", "Выйсці з поўнаэкраннага рэжыму"),
        ("Fullscreen", "Поўнаэкранны рэжым"),
        ("Mobile Actions", "Мабільныя дзеянні"),
        ("Select Monitor", "Выбраць манітор"),
        ("Control Actions", "Дзеянні па кіраванню"),
        ("Display Settings", "Налады адлюстравання"),
        ("Ratio", "Суадносіны"),
        ("Image Quality", "Якасць выявы"),
        ("Scroll Style", "Стыль пракруткі"),
        ("Show Toolbar", "Паказаць панэль інструментаў"),
        ("Hide Toolbar", "Схаваць панэль інструментаў"),
        ("Direct Connection", "Прамаое злучэнне"),
        ("Relay Connection", "Рэтрансляванае злучэнне"),
        ("Secure Connection", "Бяспечнае злучэнне"),
        ("Insecure Connection", "Нябяспечнае злучэнне"),
        ("Scale original", "Арыгінальны маштаб"),
        ("Scale adaptive", "Адаптыўны маштаб"),
        ("General", "Агульныя"),
        ("Security", "Бяспека"),
        ("Theme", "Тэма"),
        ("Dark Theme", "Цёмная тэма"),
        ("Light Theme", "Светлая тэма"),
        ("Dark", "Цёмны"),
        ("Light", "Светлы"),
        ("Follow System", "Прытрымлівацца сістэмы"),
        ("Enable hardware codec", "Уключыць апаратны кодэк"),
        ("Unlock Security Settings", "Разблакаваць налады бяспекі"),
        ("Enable audio", "Уключыць перадачу гуку"),
        ("Unlock Network Settings", "Разблакаваць сеткавыя налады"),
        ("Server", "Сервер"),
        ("Direct IP Access", "Прамы IP-доступ"),
        ("Proxy", "Проксі"),
        ("Apply", "Прымяніць"),
        ("Disconnect all devices?", "Адключыць усе прылады?"),
        ("Clear", "Ачысціць"),
        ("Audio Input Device", "Прылада ўводу гуку"),
        ("Use IP Whitelisting", "Выкарыстоўваць белы спіс IP"),
        ("Network", "Сетка"),
        ("Pin Toolbar", "Закрэпіць панэль інструментаў"),
        ("Unpin Toolbar", "Адкрэпіць панэль інструментаў"),
        ("Recording", "Запіс"),
        ("Directory", "Тэчка"),
        ("Automatically record incoming sessions", "Аўтаматычна запісваць уваходныя сесіі"),
        ("Change", "Змяніць"),
        ("Start session recording", "Пачаць запіс сесіі"),
        ("Stop session recording", "Спыніць запіс сесіі"),
        ("Enable recording session", "Уключыць запіс сесіі"),
        ("Enable LAN discovery", "Уключыць выяўленне ў лакальнай сетцы"),
        ("Deny LAN discovery", "Забараніць выяўленне ў лакальнай сетцы"),
        ("Write a message", "Напісаць паведамленне"),
        ("Prompt", "Падказка"),
        ("Please wait for confirmation of UAC...", "Дачакайцеся пацверджання UAC..."),
        ("elevated_foreground_window_tip", "Бягучае акно аддаленага працоўнага стала патрабуе вышэйшых прывілегій для працы, таму часова немагчыма выкарыстоўваць мыш і клавіятуру. Можна папрасіць аддаленага карыстальніка згорнуць бягучае акно або націснуць кнопку павышэння правоў у акне кіравання падлучэннем. Каб прадухіліць гэтую праблему ў будучыні, рэкамендуецца ўстанавіць праграмнае забеспячэнне на аддаленай прыладзе."),
        ("Disconnected", "Адключана"),
        ("Other", "Іншае"),
        ("Confirm before closing multiple tabs", "Пацвердзіць закрыццё некалькіх ўкладак"),
        ("Keyboard Settings", "Налады клавіятуры"),
        ("Full Access", "Поўны доступ"),
        ("Screen Share", "Дэманстрацыя экрана"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland патрабуе Ubuntu версіі 21.04 або навейшай."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Для Wayland патрабуецца вышэйшая версія дыстрыбутыву Linux. Карыстайцеся працоўным сталом X11 або зменіце сваю АС."),
        ("JumpLink", "Перайсці па спасылцы"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Выберыце экран для дэманстрацыі (кіруецца аддаленай стараной)."),
        ("Show RustDesk", "Паказаць RustDesk"),
        ("This PC", "Гэты кампутар"),
        ("or", "або"),
        ("Continue with", "Працягнуць з"),
        ("Elevate", "Павысіць"),
        ("Zoom cursor", "Павялічэнне курсора"),
        ("Accept sessions via password", "Прымаць сеансы па паролю"),
        ("Accept sessions via click", "Прымаць сеансы націскам кнопкі"),
        ("Accept sessions via both", "Прымаць сеансы па паролю і націскам кнопкі"),
        ("Please wait for the remote side to accept your session request...", "Дачакайцеся, пакуль аддаленая старана прыме ваш запыт на сеанс..."),
        ("One-time Password", "Аднаразовы пароль"),
        ("Use one-time password", "Выкарыстоўваць аднаразовы пароль"),
        ("One-time password length", "Даўжыня аднагаразовага пароля"),
        ("Request access to your device", "Запыт на доступ да вашай прылады"),
        ("Hide connection management window", "Схаваць акно кіравання падлучэннямі"),
        ("hide_cm_tip", "Дазваляць схаванне акна ў выпадку, калі прымаюцца сесіі па паролю або выкарыстоўваецца пастаянны пароль"),
        ("wayland_experiment_tip", "Падтрымка Wayland знаходзіцца на эксперыментальнай стадыі, калі вам неабходны аўтаматычны доступ, выкарыстоўвайце X11."),
        ("Right click to select tabs", "Правы клік для выбару ўкладак"),
        ("Skipped", "Прапушчана"),
        ("Add to address book", "Дадаць у адрасную кнігу"),
        ("Group", "Група"),
        ("Search", "Пошук"),
        ("Closed manually by web console", "Закрыта ўручную праз вэб-кансоль"),
        ("Local keyboard type", "Тып лакальнай клавіятуры"),
        ("Select local keyboard type", "Выберыце тып лакальнай клавіятуры"),
        ("software_render_tip", "Калі ў вас ёсць відэакарта Nvidia і аддаленае акно зачыняецца адразу пасля падлучэння, магчыма, дапаможа ўстаноўка драйвера Nouveau і выбар выкарыстання праграмнай візуалізацыі. Патрабуецца перазагрузка."),
        ("Always use software rendering", "Заўсёды выкарыстоўваць праграмную візуалізацыю"),
        ("config_input", "Каб кіраваць аддаленым працоўным сталом праз клавіятуру, неабходна дазволіць RustDesk маніторынг уводу."),
        ("config_microphone", "Каб размаўляць з аддаленай старонкай, неабходна дазволіць RustDesk запіс аўдыё."),
        ("request_elevation_tip", "Таксама можна запытаць павышэнне правоў, калі хто-небудзь знаходзіцца на аддаленай старонцы."),
        ("Wait", "Чакайце"),
        ("Elevation Error", "Памылка павышэння правоў"),
        ("Ask the remote user for authentication", "Запытаць аўтэнтыфікацыю ў аддаленага карыстальніка"),
        ("Choose this if the remote account is administrator", "Выберыце гэта, калі аддалены акаўнт з'яўляецца адміністратарам"),
        ("Transmit the username and password of administrator", "Перадаць імя карыстальніка і пароль адміністратара"),
        ("still_click_uac_tip", "Дагэтуль патрэбна, каб аддалены карыстальнік націснуў \"OK\" ў акне UAC пры запуску RustDesk."),
        ("Request Elevation", "Запыт на павышэнне"),
        ("wait_accept_uac_tip", "Пачакайце, пакуль аддалены карыстальнік пацвердзіць запыт UAC."),
        ("Elevate successfully", "Павышэнне паспяхова выканана"),
        ("uppercase", "Вялікія літары"),
        ("lowercase", "Малыя літары"),
        ("digit", "Лічбы"),
        ("special character", "Спецыяльныя сімвалы"),
        ("length>=8", "Даўжыня >= 8 сімвалаў"),
        ("Weak", "Слабы"),
        ("Medium", "Сярэдні"),
        ("Strong", "Моцны"),
        ("Switch Sides", "Пераключыць бакі"),
        ("Please confirm if you want to share your desktop?", "Пацвердзіце, калі хочаце дазволіць паказ вашага працоўнага стала?"),
        ("Display", "Адлюстраванне"),
        ("Default View Style", "Стыль адлюстравання па змаўчанні"),
        ("Default Scroll Style", "Стыль пракруткі па змаўчанні"),
        ("Default Image Quality", "Якасць выявы па змаўчанні"),
        ("Default Codec", "Кодэк па змаўчанні"),
        ("Bitrate", "Бітрэйт"),
        ("FPS", "Колькасць кадраў у секунду"),
        ("Auto", "Аўта"),
        ("Other Default Options", "Іншыя параметры па змаўчанні"),
        ("Voice call", "Галасавы выклік"),
        ("Text chat", "Тэкставы чат"),
        ("Stop voice call", "Спыніць галасавы выклік"),
        ("relay_hint_tip", "Непасрэднае падключэнне можа быць немагчымым. У гэтым выпадку можна спрабаваць падключыцца праз рэлей.\nАкрамя таго, калі вы хочаце адразу выкарыстоўваць рэлей, можна дадаць да ідэнтыфікатара суфікс \"/r\" або ўключыць \"Заўсёды падключацца праз рэлей\" ў наладах аддаленага вузла."),
        ("Reconnect", "Перападключыць"),
        ("Codec", "Кодэк"),
        ("Resolution", "Разрознасць"),
        ("No transfers in progress", "Перадача не ажыццяўляецца"),
        ("Set one-time password length", "Усталяваць даўжыню аднаразовага пароля"),
        ("RDP Settings", "Налады RDP"),
        ("Sort by", "Сартаваць па"),
        ("New Connection", "Новае злучэнне"),
        ("Restore", "Аднавіць"),
        ("Minimize", "Згарнуць"),
        ("Maximize", "Разгарнуць"),
        ("Your Device", "Ваша прылада"),
        ("empty_recent_tip", "Няма апошніх сеансаў!\nЧас запланаваць новы."),
        ("empty_favorite_tip", "Яшчэ няма выбраных аддаленых вузлоў?\nДавайце знойдзем, каго можна дадаць у выбранае."),
        ("empty_lan_tip", "Не знойдзены аддаленыя вузлы."),
        ("empty_address_book_tip", "У адраснай кнізе няма аддаленых вузлоў."),
        ("eg: admin", "напрыклад: admin"),
        ("Empty Username", "Пустае імя карыстальніка"),
        ("Empty Password", "Пусты пароль"),
        ("Me", "Я"),
        ("identical_file_tip", "Файл ідэнтычны файлу на аддаленым вузле"),
        ("show_monitors_tip", "Паказваць маніторы на панэлі інструментаў"),
        ("View Mode", "Рэжым прагляду"),
        ("login_linux_tip", "Каб ўключыць сеанс працоўнага стала X, неабходна ўвайсці ў аддалены акаўнт Linux."),
        ("verify_rustdesk_password_tip", "Пацвердзіць пароль RustDesk"),
        ("remember_account_tip", "Запомніць гэты акаўнт"),
        ("os_account_desk_tip", "Гэты акаўнт выкарыстоўваецца для ўваходу ў аддаленую аперацыйную сістэму і ўключэння сеансу працоўнага сталу ў рэжыме headless."),
        ("OS Account", "Акаўнт АС"),
        ("another_user_login_title_tip", "Іншы карыстальнік ўжо ўвайшоў у сістэму"),
        ("another_user_login_text_tip", "Адключыць"),
        ("xorg_not_found_title_tip", "Xorg не знойдзены"),
        ("xorg_not_found_text_tip", "Усталюйце Xorg"),
        ("no_desktop_title_tip", "Няма даступных працоўных сталоў"),
        ("no_desktop_text_tip", "Усталюйце GNOME Desktop"),
        ("No need to elevate", "Павышэнне правоў не патрабуецца"),
        ("System Sound", "Сістэмны гук"),
        ("Default", "Па змаўчанні"),
        ("New RDP", "Новы RDP"),
        ("Fingerprint", "Адбітак"),
        ("Copy Fingerprint", "Капіраваць адбітак"),
        ("no fingerprints", "адбіткі адсутнічаюць"),
        ("Select a peer", "Выберыце аддалены ўзел"),
        ("Select peers", "Выберыце аддаленыя ўзлы"),
        ("Plugins", "Плагіны"),
        ("Uninstall", "Выдаліць"),
        ("Update", "Абнавіць"),
        ("Enable", "Уключыць"),
        ("Disable", "Адключыць"),
        ("Options", "Параметры"),
        ("resolution_original_tip", "Арыгінальнае разознасць"),
        ("resolution_fit_local_tip", "Супадзенне з лакальнай разрознасцю"),
        ("resolution_custom_tip", "Карыстацкая разрознасць"),
        ("Collapse toolbar", "Згарнуць панэль інструментаў"),
        ("Accept and Elevate", "Прыняць і павысіць"),
        ("accept_and_elevate_btn_tooltip", "Дазволіць падлучэнне і павысіць правы UAC."),
        ("clipboard_wait_response_timeout_tip", "Час чакання адказу капіравання буфера абмену скончыўся"),
        ("Incoming connection", "Уваходнае падлучэнне"),
        ("Outgoing connection", "Выходнае падлучэнне"),
        ("Exit", "Выхад"),
        ("Open", "Адкрыць"),
        ("logout_tip", "Вы сапраўды жадаеце выйсці?"),
        ("Service", "Служба"),
        ("Start", "Запусціць"),
        ("Stop", "Спыніць"),
        ("exceed_max_devices", "Дасягнута максімальная колькасць кіруемых прылад."),
        ("Sync with recent sessions", "Сінхранізацыя з апошнімі сеансамі"),
        ("Sort tags", "Сартаваць тэгі"),
        ("Open connection in new tab", "Адкрыць падлучэнне ў новай ўкладцы"),
        ("Move tab to new window", "Перамясціць ўкладку ў новае акно"),
        ("Can not be empty", "Ня можа быць пустым"),
        ("Already exists", "Ужо існуе"),
        ("Change Password", "Змяніць пароль"),
        ("Refresh Password", "Абнавіць пароль"),
        ("ID", "ID"),
        ("Grid View", "Сетка"),
        ("List View", "Спіс"),
        ("Select", "Выбар"),
        ("Toggle Tags", "Пераключыць тэгі"),
        ("pull_ab_failed_tip", "Немагчыма абнавіць адрасную кнігу"),
        ("push_ab_failed_tip", "Немагчыма сінхранізаваць адрасную кнігу з серверам"),
        ("synced_peer_readded_tip", "Прылады, якія былі на апошніх сеансах, будуць сінхранізаваны з адраснай кнігай."),
        ("Change Color", "Змяніць колер"),
        ("Primary Color", "Асноўны колер"),
        ("HSV Color", "Колер HSV"),
        ("Installation Successful!", "Інсталяцыя прайшла паспяхова!"),
        ("Installation failed!", "Інсталяцыя не ўдалася!"),
        ("Reverse mouse wheel", "Рэверс кола мышы"),
        ("{} sessions", "{} сеансаў"),
        ("scam_title", "Вы можаце быць АБМАНУТЫ!"),
        ("scam_text1", "Калі вы размаўляеце па тэлефоне з кімсці, каго вы НЕ ВЕДАЕЦЕ і каму НЕ ДАВЕРАЕЦЕ, і ён просіць вас выкарыстаць RustDesk і запусціць яго службу, не працягвайце і неадкладна адмяніце размову."),
        ("scam_text2", "Магчыма, гэта аферыст, які паспрабуе ўкрасць вашыя грошы або іншую асабістую інфармацыю."),
        ("Don't show again", "Не паказваць больш"),
        ("I Agree", "Я згодны"),
        ("Decline", "Адхіліць"),
        ("Timeout in minutes", "Час чакання (у хвілінах)"),
        ("auto_disconnect_option_tip", "Аўтаматычна зачыняць уваходныя сеансы пры неактыўнасці карыстальніка"),
        ("Connection failed due to inactivity", "Падлучэнне не ўдалося з-за неактыўнасці"),
        ("Check for software update on startup", "Праверка абнаўленняў праграмы пры запуску"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Абнавіце RustDesk Server Pro да версіі {} або новейшай!"),
        ("pull_group_failed_tip", "Немагчыма абнавіць групу"),
        ("Filter by intersection", "Фільтраваць па перасячэнні"),
        ("Remove wallpaper during incoming sessions", "Схаваць фон працоўнага стала падчас ўваходнага сеансу"),
        ("Test", "Тэст"),
        ("display_is_plugged_out_msg", "Дысплей адключаны, пераключыцеся на першы дысплей."),
        ("No displays", "Няма дысплеяў"),
        ("elevated_switch_display_msg", "Пераключыцеся на асноўны дысплей, бо ў павышаным рэжыме не падтрымліваецца некалькі дысплеяў."),
        ("Open in new window", "Адкрыць у новым акне"),
        ("Show displays as individual windows", "Паказваць дысплеі ў асобных акнах"),
        ("Use all my displays for the remote session", "Выкарыстоўваць усе мае дысплеі для аддаленага сеансу"),
        ("selinux_tip", "На вашай прыладзе ўключаны SELinux, што можа перашкаджаць правільнай працы RustDesk на кіруючым баку."),
        ("Change view", "Змяніць выгляд"),
        ("Big tiles", "Вялікія пліткі"),
        ("Small tiles", "Маленькія пліткі"),
        ("List", "Спіс"),
        ("Virtual display", "Віртуальны дысплей"),
        ("Plug out all", "Адключыць усё"),
        ("True color (4:4:4)", "True color (4:4:4)"),
        ("Enable blocking user input", "Дазволіць блакаванне ўводу карыстальніка на прыладзе"),
        ("id_input_tip", "Можна ўвесці ідэнтыфікатар, просты IP-адрас або дамен з портам (<дамен>:<порт>).\nКаб атрымаць доступ да прылады на іншым серверы, дадайце адрас сервера (<id>@<адрас_сервера>?key=<ключ_значэнне>), напрыклад:\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nКалі неабходна атрымаць доступ да прылады на грамадскім серверы, увядзіце \"<id>@public\", ключ для грамадскага сервера не патрабуецца."),
        ("privacy_mode_impl_mag_tip", "Рэжым 1"),
        ("privacy_mode_impl_virtual_display_tip", "Рэжым 2"),
        ("Enter privacy mode", "Уключыць рэжым канфідэнцыяльнасці"),
        ("Exit privacy mode", "Адключыць рэжым канфідэнцыяльнасці"),
        ("idd_not_support_under_win10_2004_tip", "Драйвер непрамога адлюстравання не падтрымліваецца. Патрабуецца Windows 10 версіі 2004 ці навейшая."),
        ("switch_display_elevated_connections_tip", "Пераключэнне на неасноўны дысплей не падтрымліваецца ў рэжыме павышэння правоў пры наяўнасці некалькіх падлучэнняў. Паўтарыце спробу пасля ўстаноўкі, калі хочаце кіраваць некалькімі дысплеямі."),
        ("input_source_1_tip", "Крыніца ўводу 1"),
        ("input_source_2_tip", "Крыніца ўводу 2"),
        ("capture_display_elevated_connections_tip", "Захоп экрана некалькіх дысплеяў не падтрымліваецца ў рэжыме павышэння правоў. Паўтарыце спробу пасля ўстаноўкі, калі хочаце кіраваць некалькімі дысплеямі."),
        ("Swap control-command key", "Памяняць месцамі значэнні кнопак Ctrl і Command"),
        ("swap-left-right-mouse", "Памяняць месцамі значэнні левай і правай кнопак мышы"),
        ("2FA code", "Код двухфактарнай аўтэнтыфікацыі"),
        ("More", "Яшчэ"),
        ("enable-2fa-title", "Выкарыстоўваць двухфактарную аўтэнтыфікацыю"),
        ("enable-2fa-desc", "Наладзьце праграму аўтэнтыфікацыі. Выкарыстоўвайце, напрыклад, Authy, Microsoft або Google Authenticator на тэлефоне ці кампутары.\n\nСкануйце QR-код з дапамогай праграмы аўтэнтыфікацыі і ўвядзіце код, які пакажа гэта праграма, каб уключыць двухфактарную аўтэнтыфікацыю."),
        ("wrong-2fa-code", "Немагчыма пацвердзіць код. Праверце код і налады мясцовага часу."),
        ("enter-2fa-title", "Двухфактарная аутэнтыфікацыя"),
        ("Email verification code must be 6 characters.", "Код верыфікацыі па электроннай пошце павінен складацца з 6 сімвалаў."),
        ("2FA code must be 6 digits.", "Код двухфактарнай аутэнтыфікацыі павінен складацца з 6 лічбаў."),
        ("Multiple Windows sessions found", "Знойдзена некалькі сеансаў Windows"),
        ("Please select the session you want to connect to", "Выберыце сеанс, да якога вы жадаеце падключыцца"),
        ("powered_by_me", "На аснове RustDesk"),
        ("outgoing_only_desk_tip", "Гэта спецыялізаваная версія.\nВы можаце падключацца да іншых прылад, але іншыя прылады не могуць падключацца да вашай."),
        ("preset_password_warning", "Гэта спецыялізаваная версія з устаноўленым загадзя паролем. Любы, хто ведае гэты пароль, можа атрымаць поўны кантроль над вашай прыладай. Калі гэта для вас нечакана, адразу выдаліце гэта праграмнае забеспячэнне."),
        ("Security Alert", "Папярэджанне аб бяспецы"),
        ("My address book", "Мая адрасная кніга"),
        ("Personal", "Асабісты"),
        ("Owner", "Уладальнік"),
        ("Set shared password", "Устанавіць агульны пароль"),
        ("Exist in", "Існуе ў"),
        ("Read-only", "Толькі для чытання"),
        ("Read/Write", "Чытанне і запіс"),
        ("Full Control", "Поўны кантроль"),
        ("share_warning_tip", "Палі вышэй з'яўляюцца агульнымі і бачнымі іншым."),
        ("Everyone", "Усе"),
        ("ab_web_console_tip", "Больш у вэб-кансолі"),
        ("allow-only-conn-window-open-tip", "Дазволіць толькі падключэнне пры адкрытым акне RustDesk"),
        ("no_need_privacy_mode_no_physical_displays_tip", "Фізічныя дысплеі адсутнічаюць, няма патрэбы выкарыстоўваць рэжым канфідэнцыяльнасці."),
        ("Follow remote cursor", "Сачыць за аддаленага курсарам"),
        ("Follow remote window focus", "Сачыць за фокусам аддаленага акна"),
        ("default_proxy_tip", "Пратакол і порт па змаўчанні: Socks5 і 1080"),
        ("no_audio_input_device_tip", "Прылада ўваходнага аудыё не знойдзена."),
        ("Incoming", "Уваходныя"),
        ("Outgoing", "Выходныя"),
        ("Clear Wayland screen selection", "Адмяніць выбар экрана Wayland"),
        ("clear_Wayland_screen_selection_tip", "Пасля адмены можна зноў выбраць экран для дэманстрацыі."),
        ("confirm_clear_Wayland_screen_selection_tip", "Адмяніць выбар экрана Wayland?"),
        ("android_new_voice_call_tip", "Атрыман новы запыт на галасавы выклік. Калі вы прымеце яго, гук пераключыцца на галасавае злучэнне."),
        ("texture_render_tip", "Выкарыстоўваць візуалізацыю тэкстураў для павышэння каб плаўнасці выявы."),
        ("Use texture rendering", "Візуалізацыя тэкстураў"),
        ("Floating window", "Плавучае акно"),
        ("floating_window_tip", "Дапамагае падтрымліваць фонавую службу RustDesk"),
        ("Keep screen on", "Трымаць экран уключаным"),
        ("Never", "Ніколі"),
        ("During controlled", "Пры кіраванні"),
        ("During service is on", "Пры запушчанай службе"),
        ("Capture screen using DirectX", "Захоп экрана з выкарыстаннем DirectX"),
        ("Back", ""),
        ("Apps", ""),
        ("Volume up", ""),
        ("Volume down", ""),
        ("Power", ""),
    ].iter().cloned().collect();
}