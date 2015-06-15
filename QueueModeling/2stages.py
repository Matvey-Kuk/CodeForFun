import random

from framework import *


class GeneratorStation(Station, OutputForStation):

    def __init__(self, name, probability_of_generation):
        Station.__init__(self, name)
        OutputForStation.__init__(self)
        self.__probability_of_generation = probability_of_generation
        self.__rejected_queue = Queue()
        self.__generated_counter = 0
        self.clock()

    def clock(self):
        current_time = 0
        while current_time < EventHorizon.time_limit:
            current_time += random.expovariate(1/self.__probability_of_generation)
            EventHorizon.register_callback_in_time(current_time, self.throw_away_callback, Car(''))

    def throw_away_callback(self, time, data):
        if len(self._transacts_ready_for_output) > 0:
            self.__rejected_queue.push_transact(data)
        else:
            self.__generated_counter += 1
            self._transacts_ready_for_output.append(data)

    def __repr__(self):
        return Station.__repr__(self) + " Транзакты, не попавшие далее: " + repr(self.__rejected_queue)

    def statistics(self):
        return "Не попали в обработку: " + repr(self.__rejected_queue.get_amount_of_transacts()) + " Всего сгенерировано: " + repr(self.__generated_counter + self.__rejected_queue.get_amount_of_transacts())




class OrdinaryStation(Station, InputOutputForStation):
    def __init__(self, name, delay):
        Station.__init__(self, name)
        InputOutputForStation.__init__(self)
        self.__queue = DelayedQueue(None, delay)

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


global_environment = Environment()

generator_station = GeneratorStation("Генератор", 120)
station_1 = OrdinaryStation("Станция 1", 240)
station_2 = OrdinaryStation("Станция 2", 110)
exit_station = ExitStation("Выход")

global_environment.add_inner_environment(generator_station)
global_environment.add_inner_environment(station_1)
global_environment.add_inner_environment(station_2)
global_environment.add_inner_environment(exit_station)


def paint_statistics(time, data):
    print(global_environment)
    print(global_environment.statistics())

EventHorizon.register_callback_in_time(480, paint_statistics, None)
EventHorizon.execute_callbacks()