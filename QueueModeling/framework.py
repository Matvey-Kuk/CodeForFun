import random


class TransactCannotBePassedException(Exception):
    pass


class Car:
    counter = 0

    def __init__(self, marks):
        self.name = str(Car.counter)
        Car.counter += 1
        self.marks = marks

    def __repr__(self):
        return self.name + " Marks:" + repr(self.marks)


class Queue(object):
    def __init__(self):
        self._transacts = []
        self._clocks_counter = 0
        self._transacts_len_counter = 0
        self._max_transacts_len = 0

    def push_transact(self, transact):
        self._transacts.append(transact)

    def get_transact(self, remove_after_getting=True):
        if remove_after_getting:
            return self._transacts.pop()
        else:
            if len(self._transacts) > 0:
                return self._transacts[len(self._transacts) - 1]

    def __repr__(self):
        return repr(self._transacts)

    def ready_to_transacts_get(self):
        return len(self._transacts) > 0

    def get_amount_of_transacts(self):
        return len(self._transacts)

    def clock(self):
        self._clocks_counter += 1
        self._transacts_len_counter += len(self._transacts)

        if self.get_amount_of_transacts() > self._max_transacts_len:
                    self._max_transacts_len = self.get_amount_of_transacts()

    def get_average_amount_of_transacts(self):
        return self._transacts_len_counter / self._clocks_counter

    def get_max_query_len(self):
        return self._max_transacts_len


class QueueWithCapacity(Queue):
    def __init__(self, capacity):
        Queue.__init__(self)
        self._capacity = capacity

    def push_transact(self, transact):
        if QueueWithCapacity.ready_to_transacts_push(self):
            Queue.push_transact(self, transact)
        else:
            raise TransactCannotBePassedException()

    def ready_to_transacts_push(self):
        if self._capacity is None:
            return True
        return len(self._transacts) <= self._capacity


class DelayedQueue(QueueWithCapacity):
    def __init__(self, capacity, delay):
        super(DelayedQueue, self).__init__(capacity)
        self.__blocked_transacts = []
        self.__delay = delay

    def push_transact(self, transact):
        self.__blocked_transacts.append({
            "transact": transact,
            "ticks_spent": 0
        })

    def get_amount_of_transacts(self):
        return Queue.get_amount_of_transacts(self) + len(self.__blocked_transacts)

    def ready_to_transacts_push(self):
        if self._capacity is None:
            return True
        return self.get_amount_of_transacts() < self._capacity

    def clock(self):
        Queue.clock(self)
        self._transacts_len_counter += len(self.__blocked_transacts)

        new_blocked_transacts = []
        for transact_with_tick_counter in self.__blocked_transacts:
            transact_with_tick_counter["ticks_spent"] += 1
            if transact_with_tick_counter["ticks_spent"] >= self.__delay:
                super(DelayedQueue, self).push_transact(transact_with_tick_counter["transact"])
            else:
                new_blocked_transacts.append(transact_with_tick_counter)
        self.__blocked_transacts = new_blocked_transacts

    def __repr__(self):
        return "In process: " + repr(self.__blocked_transacts) + " Ready: " + repr(self._transacts)


class Station(object):
    def __init__(self, name):
        self.name = name

    def __repr__(self):
        return "Название станции: \"" + self.name + "\""

    def clock(self):
        pass


class OutputForStation():
    def __init__(self):
        self._transacts_ready_for_output = []

    def get_transact(self):
        if self.ready_to_get_transacts():
            return self._transacts_ready_for_output.pop()

    def ready_to_get_transacts(self):
        return len(self._transacts_ready_for_output) > 0


class InputForStation():
    def __init__(self):
        self._transacts_got_from_input = []

    def push_transact(self, transact):
        self._transacts_got_from_input.append(transact)

    def ready_to_transacts_push(self):
        return True


class InputOutputForStation(InputForStation, OutputForStation):
    def __init__(self):
        InputForStation.__init__(self)
        OutputForStation.__init__(self)


class Environment():
    """
    "Среда", в которой существуют другие "среды". Отвечает за опрос внутренних "сред" и перемещение транзактов
    от одной к другой. На границах получает транзакты в push_transacts и отдает в get_transacts.
    Реализует вложенность.
    """
    def __init__(self):
        self._inner_environments = []

    def add_inner_environment(self, new_inner_environment):
        self._inner_environments.append(new_inner_environment)

    def clock(self):
        for environment in self._inner_environments:
            environment.clock()
        self.transport_transacts()

    def push_transacts(self, transacts):
        if len(self._inner_environments) > 0:
            self._inner_environments[0].push_transacts(transacts)
        else:
            raise Exception('Method must be overridden if no inner environments are available.')

    def get_transact(self, remove_after_getting=True):
        if len(self._inner_environments) > 0:
            return self._inner_environments[len(self._inner_environments) - 1].get_transact(remove_after_getting)
        else:
            raise Exception('Method must be overridden if no inner environments are available.')

    def ready_to_get_transacts(self):
        if len(self._inner_environments) > 0:
            return self._inner_environments[len(self._inner_environments) - 1].ready_to_get_transacts()
        else:
            raise Exception('Method must be overridden if no inner environments are available.')

    def transport_transacts(self):
        for inner_environment_number, inner_environment in enumerate(self._inner_environments, start=0):
            if inner_environment_number < len(self._inner_environments) - 1:
                while self._inner_environments[inner_environment_number + 1].ready_to_transacts_push() and \
                        inner_environment.ready_to_get_transacts():
                    transact = inner_environment.get_transact()
                    self._inner_environments[inner_environment_number + 1].push_transact(transact)

    def __repr__(self):
        return repr(self._inner_environments)

    def get_amount_of_transacts(self):
        amount = 0
        for environment in self._inner_environments:
            amount += environment.get_amount_of_transacts()
        return amount

    def statistics(self):
        statistics = ""
        for environment in self._inner_environments:
            statistics += " " + environment.statistics()
        return statistics


class EnvironmentWithRouter(Environment):

    def __init__(self, router_settings):
        Environment.__init__(self)
        #Метка транзакта к типу станции
        self.__router_settings = router_settings

    def approve_route(self, transact, station):
        return self.__router_settings[transact.marks] == type(station)

    def push_transact(self, transact):
        succeed = False
        for inner_environment in self._inner_environments:
            if self.approve_route(transact, inner_environment):
                succeed = True
                self._inner_environments[0].push_transact(transact)
        if not succeed:
            raise Exception('Router can not find target to push transact in environment.')


class ExitStation(Station, InputForStation):
    def __init__(self, name):
        Station.__init__(self, name)
        InputForStation.__init__(self)
        self.__queue = Queue()

    def __repr__(self):
        return Station.__repr__(self) + " Транзакты:" + repr(self.__queue)

    def push_transact(self, transact):
        self.__queue.push_transact(transact)

    def clock(self):
        self.__queue.clock()

    def statistics(self):
        return Station.__repr__(self) + " транзактов: " + repr(self.__queue.get_amount_of_transacts())