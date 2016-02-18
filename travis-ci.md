# Travis-Ci
[Travis-ci Start]  


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

    Encrypted environment variables are not available to pull requests from forks due to the security risk of exposing such information to unknown code.

Encrypting Variables Using a Public Key #

Encrypt environment variables using the public key attached to your repository using the travis gem:

gem install travis
cd my_project
travis encrypt MY_SECRET_ENV=super_secret

To automatically add the encrypted environment variable to your .travis.yml:

travis encrypt MY_SECRET_ENV=super_secret --add env.matrix

    Encryption and decryption keys are tied to the repository. If you fork a project and add it to Travis CI, it will have different keys to the original.

The encryption scheme is explained in more detail in Encryption keys.
Convenience Variables #

To make using encrypted environment variables easier, the following environment variables are available:

    TRAVIS_SECURE_ENV_VARS “true” or “false” depending on the availability of environment variables
    TRAVIS_PULL_REQUEST the pull request number if the current job is a pull request, or “false” if it’s not a pull request.

Default Environment Variables #

The following default environment variables are available to all builds.

    CI=true
    TRAVIS=true
    CONTINUOUS_INTEGRATION=true
    DEBIAN_FRONTEND=noninteractive
    HAS_JOSH_K_SEAL_OF_APPROVAL=true
    USER=travis (do not depend on this value)
    HOME=/home/travis (do not depend on this value)
    LANG=en_US.UTF-8
    LC_ALL=en_US.UTF-8
    RAILS_ENV=test
    RACK_ENV=test
    MERB_ENV=test
    JRUBY_OPTS="--server -Dcext.enabled=false -Xcompile.invokedynamic=false"
    JAVA_HOME is set to the appropriate value.

Additionally, Travis CI sets environment variables you can use in your build, e.g. to tag the build, or to run post-build deployments.

    TRAVIS_BRANCH:For builds not triggered by a pull request this is the name of the branch currently being built; whereas for builds triggered by a pull request this is the name of the branch targeted by the pull request (in many cases this will be master).
    TRAVIS_BUILD_DIR: The absolute path to the directory where the repository being built has been copied on the worker.
    TRAVIS_BUILD_ID: The id of the current build that Travis CI uses internally.
    TRAVIS_BUILD_NUMBER: The number of the current build (for example, “4”).
    TRAVIS_COMMIT: The commit that the current build is testing.
    TRAVIS_COMMIT_RANGE: The range of commits that were included in the push or pull request.
    TRAVIS_JOB_ID: The id of the current job that Travis CI uses internally.
    TRAVIS_JOB_NUMBER: The number of the current job (for example, “4.1”).
    TRAVIS_OS_NAME: On multi-OS builds, this value indicates the platform the job is running on. Values are linux and osx currently, to be extended in the future.
    TRAVIS_PULL_REQUEST: The pull request number if the current job is a pull request, “false” if it’s not a pull request.
    TRAVIS_REPO_SLUG: The slug (in form: owner_name/repo_name) of the repository currently being built. (for example, “travis-ci/travis-build”).
    TRAVIS_SECURE_ENV_VARS: Whether or not encrypted environment vars are being used. This value is either “true” or “false”.
    TRAVIS_TEST_RESULT: is set to 0 if the build is successful and 1 if the build is broken.
    TRAVIS_TAG: If the current build for a tag, this includes the tag’s name.

Language-specific builds expose additional environment variables representing the current version being used to run the build. Whether or not they’re set depends on the language you’re using.

    TRAVIS_RUST_VERSION
  


