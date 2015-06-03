import random

from framework import *


class GeneratorStation(Station, OutputForStation):
    def __init__(self, name, probability_of_generation_for_wash, probability_of_generation_for_polish):
        Station.__init__(self, name)
        OutputForStation.__init__(self)
        self.__probability_of_generation_for_wash = probability_of_generation_for_wash
        self.__probability_of_generation_for_polish = probability_of_generation_for_polish
        self.__rejected_queue = Queue()
        self.__generated_counter = 0

    def clock(self):
        if random.random() <= self.__probability_of_generation_for_wash:
            if len(self._transacts_ready_for_output) > 0:
                self.__rejected_queue.push_transact(Car('Want wash'))
            else:
                self.__generated_counter += 1
                self._transacts_ready_for_output.append(Car('Want wash'))
        if random.random() <= self.__probability_of_generation_for_polish:
            if len(self._transacts_ready_for_output) > 0:
                self.__rejected_queue.push_transact(Car('Want polish'))
            else:
                self.__generated_counter += 1
                self._transacts_ready_for_output.append(Car('Want polish'))

    def __repr__(self):
        return Station.__repr__(self) + " Транзакты, не попавшие далее: " + repr(self.__rejected_queue)

    def statistics(self):
        return "Не попали в обработку: " + repr(self.__rejected_queue.get_amount_of_transacts()) + " Всего сгенерировано: " + repr(self.__generated_counter + self.__rejected_queue.get_amount_of_transacts())


class WashStation(Station, InputOutputForStation):

    def __init__(self, name):
        Station.__init__(self, name)
        InputOutputForStation.__init__(self)
        self.__queue = DelayedQueue(1, 4)

    def push_transact(self, transact):
        self.__queue.push_transact(transact)

    def ready_to_transacts_push(self):
        return self.__queue.ready_to_transacts_push()

    def get_transact(self, remove_after_getting=True):
        return self.__queue.get_transact(remove_after_getting)

    def ready_to_get_transacts(self):
        return self.__queue.ready_to_transacts_get()

    def __repr__(self):
        return Station.__repr__(self) + " очередь: " + repr(self.__queue)

    def clock(self):
        self.__queue.clock()

    def get_amount_of_transacts(self):
        return self.__queue.get_amount_of_transacts()

    def statistics(self):
        return Station.__repr__(self) + " средняя загрузка: " + repr(self.__queue.get_average_amount_of_transacts()) +\
            " максимальная загрузка: " + repr(self.__queue.get_max_query_len())


class PolishStation(Station, InputOutputForStation):

    def __init__(self, name):
        Station.__init__(self, name)
        InputOutputForStation.__init__(self)
        self.__queue = DelayedQueue(2, 15)

    def push_transact(self, transact):
        self.__queue.push_transact(transact)

    def ready_to_transacts_push(self):
        return self.__queue.ready_to_transacts_push()

    def get_transact(self, remove_after_getting=True):
        return self.__queue.get_transact(remove_after_getting)

    def ready_to_get_transacts(self):
        return self.__queue.ready_to_transacts_get()

    def __repr__(self):
        return Station.__repr__(self) + " очередь: " + repr(self.__queue)

    def clock(self):
        self.__queue.clock()

    def get_amount_of_transacts(self):
        return self.__queue.get_amount_of_transacts()

    def statistics(self):
        return Station.__repr__(self) + " средняя загрузка: " + repr(self.__queue.get_average_amount_of_transacts()) +\
            " максимальная загрузка: " + repr(self.__queue.get_max_query_len())


class GarageStation(Station, InputOutputForStation, EnvironmentWithRouter):
    def __init__(self, name, capacity, router_settings):
        Station.__init__(self, name)
        InputOutputForStation.__init__(self)
        EnvironmentWithRouter.__init__(self, router_settings)
        self.__queue = QueueWithCapacity(5)
        self.__capacity = capacity

        wash_station = WashStation("Мойка")
        polish_station = PolishStation("Полировка")

        self.add_inner_environment(wash_station)
        self.add_inner_environment(polish_station)

    def __repr__(self):
        return Station.__repr__(self) + " очередь: " + repr(self.__queue) \
            + " вложенные станции: " + repr(self._inner_environments)

    def clock(self):
        EnvironmentWithRouter.clock(self)
        stations_ready_to_get_transact = True
        while self.__queue.ready_to_transacts_get() and stations_ready_to_get_transact:
            stations_ready_to_get_transact = False
            for inner_environment in self._inner_environments:
                if self.__queue.get_amount_of_transacts() > 0:
                    if self.approve_route(self.__queue.get_transact(False), inner_environment):
                        if inner_environment.ready_to_transacts_push():
                            stations_ready_to_get_transact = True
                            inner_environment.push_transact(self.__queue.get_transact())

    def get_amount_of_transacts(self):
        return EnvironmentWithRouter.get_amount_of_transacts(self) + self.__queue.get_amount_of_transacts()

    def push_transact(self, transact):
        self.__queue.push_transact(transact)

    def ready_to_transacts_push(self):
        return self.__queue.ready_to_transacts_push() and self.get_amount_of_transacts() < self.__capacity

    def ready_to_get_transacts(self):
        return self._inner_environments[len(self._inner_environments) - 1].ready_to_get_transacts()

    def get_transact(self, remove_after_getting=True):
        transact = Environment.get_transact(self, remove_after_getting)
        return transact


global_environment = Environment()

generator_station = GeneratorStation("Генератор машин", 1/5, 1/30)

garage_router_settings = {
    'Want wash': WashStation,
    'Want polish': PolishStation
}
garage_station = GarageStation("Гараж", 2, garage_router_settings)
exit_station = ExitStation("Выход")

global_environment.add_inner_environment(generator_station)
global_environment.add_inner_environment(garage_station)
global_environment.add_inner_environment(exit_station)


for i in range(0, 480):
    global_environment.clock()
    print(global_environment)

print(global_environment.statistics())

"""
GARAGE STORAGE 2
WASH STORAGE 1
POLISH STORAGE 2

GENERATE(Exponential(1,0,5))
ASSIGN GOAL,1
TRANSFER ,TestPoint

GENERATE(Exponential(2,0,30))
ASSIGN GOAL,2

TestPoint TEST L Q$QPARK,30,FULL

QUEUE QPARK

ENTER GARAGE
DEPART QPARK

TEST E P$GOAL,1,PolishPoint

WashPoint ENTER WASH
ADVANCE(Exponential(3,0,4))
LEAVE WASH

PolishPoint ENTER POLISH
ADVANCE(Exponential(4,0,15))
LEAVE POLISH

LEAVE GARAGE

TERMINATE
FULL TERMINATE

GENERATE 480
TERMINATE 1
"""
#макс, среднюю длинну, коэф загрузки для устройств