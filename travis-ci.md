# Travis-Ci
[Travis-ci Start Doc](https://docs.travis-ci.com/)

[Travis robisys](https://travis-ci.org/robisys)

[travis-lint ](https://docs.travis-ci.com/user/travis-lint/) [lint.travis-ci.org] (http://lint.travis-ci.org/)

* Codedeploy 

[Codedeploy] (https://docs.travis-ci.com/user/deployment/codedeploy/)

Git #

This should also work with services you can deploy to via git.

'''
after_success:
  - eval "$(ssh-agent -s)" #start the ssh agent
  - chmod 600 .travis/deploy_key.pem # this key should have push access
  - ssh-add .travis/deploy_key.pem
  - git remote add deploy DEPLOY_REPO_URI_GOES_HERE
  - git push deploy
'''  


auf Grundlage von: [ www.matthias-zeis.com/archiv/magento-extensions-mit-travis-ci-testen](http://www.matthias-zeis.com/archiv/magento-extensions-mit-travis-ci-testen)

(kopiert am 18.02.2016 und danach angepasst)

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

 [environment-variables] (https://docs.travis-ci.com/user/environment-variables/)
 
 
 
Environment Variables

A common way to customize the build process is to use environment variables, which can be accessed from any stage in your build process.

    Defining Variables in .travis.yml
    Defining Variables in Repository Settings
    Encrypted Variables
    Default Environment Variables

    Variables defined in .travis.yml are tied to a certain commit. Changing them requires a new commit, restarting an old build uses the old values. They are also available automatically on forks of the repository. Define variables in .travis.yml that:
        are needed for the build to run and that don’t contain sensitive data. For instance, a test suite for a Ruby application might require $RACK_ENV to be set to test.
        differ per branch.
        differ per job.

    Variables defined in repository settings are the same for all builds. When you restart an old build, it uses the latest values. These variables are not automatically available to forks. Define variables in the Repository Settings that:
        differ per repository.
        contain sensitive data, such as third-party credentials.

    Notice that the values are not escaped when your builds are executed. Special characters (for bash) should be escaped accordingly.

    Use Encrypted variables for sensitive data such as authentication tokens.

    If you define a variable with the same name in .travis.yml and in the Repository Settings, the one in .travis.yml takes precedence. If you define a variable as both encrypted and unencrypted, the one defined later in the file takes precedence.

There is also a complete list of default environment variables which are present in all Travis CI environments.
Defining Variables in .travis.yml #

To define an environment variable in your .travis.yml, add the env line, for example:

env:
- DB=postgres

    Note that environment variable values may need quoting. For example, if they have asterisks (*) in them: env: PACKAGE_VERSION="1.0.*"

Defining Multiple Variables per Item #

When you specify multiple variables per item in the env array (matrix variables), one build is triggered per item.

rvm:
  - 1.9.3
  - rbx
env:
  - FOO=foo BAR=bar
  - FOO=bar BAR=foo

this configuration triggers 4 individual builds:

    Ruby 1.9.3 with FOO=foo and BAR=bar
    Ruby 1.9.3 with FOO=bar and BAR=foo
    Rubinius latest version (rbx) with FOO=foo and BAR=bar
    Rubinius latest version (rbx) with FOO=bar and BAR=foo

Global Variables #

Sometimes you may want to use environment variables that are global to the matrix, i.e. they’re inserted into each matrix row. That may include keys, tokens, URIs or other data that is needed for every build. In such cases, instead of manually adding such keys to each env line in matrix, you can use global and matrix keys to differentiate between those two cases. For example:

env:
  global:
    - CAMPFIRE_TOKEN=abc123
    - TIMEOUT=1000
  matrix:
    - USE_NETWORK=true
    - USE_NETWORK=false

triggers builds with the following env rows:

USE_NETWORK=true CAMPFIRE_TOKEN=abc123 TIMEOUT=1000
USE_NETWORK=false CAMPFIRE_TOKEN=abc123 TIMEOUT=1000

Defining Variables in Repository Settings #

To define variables in Repository Settings, make sure you’re logged in, navigate to the repository in question, choose “Settings” from the cog menu, and click on “Add new variable” in the “Environment Variables” section.
Screenshot of environment variables in settings
Environment Variables in the Repository Settings

By default, the value of these new environment variables is hidden from the export line in the logs. This corresponds to the behavior of encrypted variables in your .travis.yml.

Similarly, we do not provide these values to untrusted builds, triggered by pull requests from another repository.

As an alternative to the web interface, you can also use the CLI’s env command.
Encrypted Variables #

Variables can be encrypted so that their content is not shown in the corresponding export line in the build. This is used to provide sensitive data, like API credentials, to the Travis CI build. Encrypted variables are not added to untrusted builds such as pull requests coming from another repository.

A .travis.yml file containing encrypted variables looks like this:

env:
  global:
    - secure: <long encrypted string here>
    - TIMEOUT=1000
  matrix:
    - USE_NETWORK=true
    - USE_NETWORK=false
    - secure: <you can also put encrypted vars inside matrix>


Language-specific builds expose additional environment variables representing the current version being used to run the build. Whether or not they’re set depends on the language you’re using.

    TRAVIS_RUST_VERSION
  

[Status-anzeige ](https://docs.travis-ci.com/user/status-images/)

[Notification Benachrichtigung](https://docs.travis-ci.com/user/notifications/)

