# Travis-Ci
[Travis-ci Start]  

[Testen ](http://www.matthias-zeis.com/archiv/magento-extensions-mit-travis-ci-testen)

Travis CI als kostenlose Continuous-Integration-Plattform

Man kann CI-Server auch selbst verwalten und hosten (wir tun das für Kundenprojekte), doch manchmal bietet es sich an nicht das Rad neu zu erfinden und diese Aufgabe an andere auszulagern.

Einer der bekanntesten Anbieter in diesem Bereich ist Travis CI. Dort können öffentlich gehostete Projekte (etwa die von GitHub) kostenlos getestet werden. Premium-Pakete ermöglichen das Hosting von privaten Repositories.
GitHub-Magento-Extensions mit Travis testen

Für das Beispiel gehe ich davon aus, dass eine öffentlich auf GitHub verfügbare Extension auf travis-ci.org getestet werden soll.

Was Sie dazu benötigen:

    einen GitHub-User samt GitHub-Repository
    einen Travis-CI-Account

Repository erstellen

Im ersten Schritt wird das GitHub-Repository erstellt.Zuerst müssen wir auf GitHub das Repository erstellen. Ich wähle einen Namen aus und konfiguriere die Basis-Einstellungen. Natürlich kann ein bestehendes Repository verwendet werden, wenn schon eines vorhanden ist.
Travis CI konfigurieren

Im Repository hinterlegen wir eine Datei .travis.yml in der konfiguriert wird, welche PHP- und Magento-Versionen zu testen sind. An dieser Stelle muss man sich entscheiden, ob man ein eigenes Setup-Skript zur Installation von Magento etc. schreibt oder ein bestehendes aus der Community verwendet.

Wer schnell starten möchte, kann unter anderem MageTestStand von AOE oder MageCI von EcomDev verwenden. Ich habe mich für dieses Posting für MageTestStand entschieden. Meiner Erfahrung nach ist MageCI beim Setup der Test-Instanzen schneller, aber MageTestStand ist sehr pflegeleicht und verwendet Standard-Tools wie n98-magerun.

Wir legen also .travis.yml im Root-Verzeichnis des Repositories an:

.travis.yml wird im Hauptverzeichnis des Repositories angelegt.Die Datei habe ich ganz frech von Aoe_TemplateHints kopiert, denn das ist dort schon so schön vorkonfiguriert.

Die Konfiguration ist denkbar einfach zu verstehen: Getestet wird in PHP 5.3 bis PHP 5.6 sowie in HHVM, wobei PHP 5.6 und HHVM fehlschlagen dürfen, ohne dass der Build als Fehlschlag gilt. Bei Magento werden die Versionen CE 1.7.0.2 bis CE 1.9.0.1 herangezogen. Insgesamt werden also 20 Kombinationen von PHP und Magento getestet. Dann wird noch der Pfad zum Setup-Skript für die Installation von Magento, PHPUnit etc. angegeben und definiert, dass sowohl bei Erfolgen als auch bei Fehlern eine Mail an die angegebene Mail-Adresse geschickt wird.
Repository in Travis CI aktivieren

Auf der Travis-CI-Seite wird das Projekt aktiviert.

Nun kann man auf travis-ci.org gehen und sich über den Link rechts oben anmelden. Praktischerweise kann man sich direkt mit dem GitHub-Account anmelden.

Falls das neue Repository nach dem Login noch nicht in der Liste der Repositories vorhanden ist kann man mit einem Klick auf „Sync now“ die Aufstellung aktualisieren lassen. Hat man des Repository gefunden dann legt man den Schalter um und aktiviert so das Projekt für die Continuous Integration mit Travis CI.
Travis CI konfigurieren

Im nächsten Bildschirm wird das Projekt konfiguriert.Im nächsten Menü kann Travis konfiguriert werden. So stelle ich z.B. ein, dass Travis nur prüft, wenn eine Konfigurationsdatei vorhanden ist. Außerdem kann man praktischerweise definieren, dass Pull Requests gleich einmal vorab geprüft werden.
Der erste Build

Der erste Build schlägt fehl. Ein Klick auf die Job-Nummer offenbart Details.Nun kann man den ersten Build anstoßen. Das Projekt wird in einer Warteschlange eingereiht und kommt zumeist nach kurzer Zeit dran. Und siehe da: der erste Build schlägt fehl, wie man im Screenshot sehen kann. Ein Klick auf die Jobs („2.1“, „2.2“) offenbart Details, genauer gesagt den ganzen Output. Nachdem das dann doch ein wenig viel ist, hier nur der letzte Teil:

Der Grund für den Fehlschlag ist, dass noch keine Tests definiert worden sind.

Da haben wir auch schon das Problem:

    FAILURE: There were no test cases for the current run (Duration: 0 sec)

Sorgen wir also dafür, dass es etwas zu testen gibt.
Die Extension für das Testing bereit machen

Wechseln wir zurück zum Extension-Code und machen wir unser Modul wie in der Dokumentation von EcomDev_PHPUnit bereit für Tests.

In app/code/[codePool]/[Vendor]/[Modul]/etc/config.xml tragen wir die Erweiterung in die Liste der zu testenden Module ein:

Die Extension wird in der Konfigurationsdatei als zu testen gekennzeichnet.Zuletzt erstelle ich einen – zugegebenermaßen noch nicht bahnbrechenden – Test in der Datei app/code/[codePool]/[Vendor]/[Modul]/Test/Config/ActivationXml.php:

Der erste Unit-Test.
Die Module können übrigens mit Modman eingebunden werden. Dazu wird – die meisten Leser werden das schon kennen – eine modman-Datei in das Hauptverzeichnis des Repositories geschrieben:

Die modman-Datei für die Extension.
Zweiter Versuch

Sobald wir den Code ins GitHub-Repository pushen wird das Projekt schon wieder in die Travis-CI-Warteschlange aufgenommen. Wechseln wir zurück zu Travis CI und warten wir etwas ab.

Dann erscheinen hoffentlich grüne Häkchen:

Diesmal ist der Build erfolgreich.Geht doch!

Wie man sieht, schlagen die Tests in PHP 5.6 und HHVM fehl. Wer diese Versionen unterstützen möchte kann sich nun in die Details vertiefen und feststellen, wo das Problem liegt. Für das Beispiel reichen uns die erfolgreichen Tests in PHP 5.3 bis 5.5.
 Badge einbinden

Wer mag, kann jetzt noch ein Badge (siehe rechts oben im Screenshot) mit dem Ergebnis des letzten Builds auf seiner Website oder in seiner Readme einbinden damit jederzeit ersichtlich ist, ob die Tests aktuell durchlaufen.

Der Code zur Verlinkung des Badges steht in HTML, Markdown u.ä. bereit.
Man kann seine Badge-Graphik unter anderem per Markdown in die README-Datei der Extension einbinden.
Geschafft!

Gratuliation, die Basis ist eingerichtet. Jetzt heißt es Tests schreiben! Natürlich kann man das Projekt um andere Testarten, die Erstellung von Code-Metriken (z.B. mit Scrutinizer) erweitern und vieles mehr. Viel Erfolg dabei!

