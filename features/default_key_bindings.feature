Feature: Default key bindings

  Scenario: xplr exits with debug info when we press '#'
    Given xplr is running
    When I press "#"
    And xplr processes the tasks
    Then xplr performs "PrintAppStateAndQuit"
    And xplr quits or waits for the next event


  Scenario: xplr prints result and quits when I gress 'enter'
    Given xplr is running
    When I press "enter"
    And xplr processes the tasks
    Then xplr performs "PrintResultAndQuit"
    And xplr quits or waits for the next event
