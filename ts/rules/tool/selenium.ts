import { register } from '../../register.js';

register({
  tech: 'selenium',
  name: 'Selenium',
  type: 'tool',
  dependencies: [
    { type: 'npm', name: 'selenium' },
    { type: 'npm', name: 'selenium-webdriver' },
    { type: 'npm', name: 'webdriver-js-extender' },
    { type: 'npm', name: '@wdio/selenium-standalone-service' },
    { type: 'php', name: 'php-webdriver/webdriver' },
    { type: 'php', name: 'behat/mink-selenium2-driver' },
    { type: 'php', name: 'symfony/panther' },
    { type: 'php', name: 'instaclick/php-webdriver' },
    { type: 'php', name: 'phpunit/phpunit-selenium' },
    { type: 'php', name: 'se/selenium-server-standalone' },
    { type: 'ruby', name: 'selenium-webdriver' },
    { type: 'ruby', name: 'selenium' },
    { type: 'ruby', name: 'selenium-client' },
    { type: 'ruby', name: 'capybara-selenium' },
    { type: 'rust', name: 'thirtyfour' },
    { type: 'rust', name: 'selenium-rs' },
    { type: 'githubAction', name: 'Xotabu4/selenoid-github-action' },
    { type: 'golang', name: 'github.com/tebeka/selenium' },
    { type: 'python', name: 'selenium' },
    { type: 'docker', name: 'selenium/standalone-chrome' },
    { type: 'docker', name: 'selenium/node-chrome' },
    { type: 'docker', name: 'selenium/hub' },
    { type: 'docker', name: 'selenium/node-firefox' },
    { type: 'docker', name: 'selenium/standalone-firefox' },
    { type: 'docker', name: 'selenium/standalone-chrome-debug' },
    { type: 'docker', name: 'selenium/node-chrome-debug' },
    { type: 'docker', name: 'selenium/base' },
    { type: 'docker', name: 'selenium/node-edge' },
    { type: 'docker', name: 'selenium/node-firefox-debug' },
    { type: 'docker', name: 'selenium/standalone-firefox-debug' },
    { type: 'docker', name: 'selenium/node-opera' },
  ],
});
