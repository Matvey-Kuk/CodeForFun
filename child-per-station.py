children_amount = 143
stations = 10
iterations = 6


class Child():
    def __init__(self):
        self.__known_children = []
        self.__station_visits_log = []

    def been_on_station(self, station):
        return station in self.__station_visits_log

    def register_child_meeting(self, child):
        if not child in self.__known_children:
            self.__known_children.append(child)
            child.register_child_meeting(self)

    def register_station_visit(self, station):
        self.__station_visits_log.append(station)

    def knows(self, child):
        return child in self.__known_children

    def get_station_for_iteration(self, iteration):
        return self.__station_visits_log[iteration]

    def __str__(self):
        string = ""
        i = 0
        for station in self.__station_visits_log:
            string += str(station)
            i += 1
            if not i == iterations:
                string += " -> "
        return string

a = Child()
b = Child()
assert(not a.knows(b))

a.register_child_meeting(b)
assert(a.knows(b))
assert(b.knows(a))


class Station():
    def __init__(self, number):
        self.__number = number
        self.__children = []

    def set_child(self, child):
        for child_in_station in self.__children:
            child.register_child_meeting(child_in_station)
        self.__children.append(child)
        child.register_station_visit(self)

    def amount_of_known_children_inside(self, child):
        amount = 0
        for child_in_station in self.__children:
            if child.knows(child_in_station):
                amount += 1
        return amount

    def amount_of_children_inside(self):
        return len(self.__children)

    def clear(self):
        self.__children = []

    def __repr__(self):
        return str(self.__number + 1)

a = Child()
b = Child()
st = Station(1)

assert(not a.been_on_station(st))
assert(not b.been_on_station(st))

st.set_child(a)

assert(a.been_on_station(st))

st.set_child(b)

assert(b.been_on_station(st))
assert(1 == st.amount_of_known_children_inside(a))
assert(1 == st.amount_of_known_children_inside(b))

st.clear()

assert(0 == st.amount_of_known_children_inside(b))


class Camp():
    def __init__(self):
        self.__children = []
        for i in range(0, children_amount):
            self.__children.append(Child())

        self.__stations = []
        for i in range(0, stations):
            self.__stations.append(Station(i))

        self.__current_iteration = 0

    def print(self):
        for child in self.__children:
            print(child)

    def checkup_stations_load(self):
        for i in range(0, iterations):
            print("Итерация: " + str(i + 1))
            stations_load = {}
            for child in self.__children:
                target_station = child.get_station_for_iteration(i)
                if not target_station in stations_load:
                    stations_load[target_station] = 0
                stations_load[target_station] += 1

            for station in self.__stations:
                print("Станция: " + repr(station) + " Загрузка: " + repr(stations_load[station]))

    def clock(self):
        for time_moment in range(0, iterations):
            for child in self.__children:
                station_with_less_amount_of_known_children = self.__stations[0]
                stations_with_same_amount_of_known_children_as_less = []
                for station in self.__stations:
                    if station.amount_of_known_children_inside(child) < \
                            station_with_less_amount_of_known_children.amount_of_known_children_inside(child) and \
                            not child.been_on_station(station):
                        station_with_less_amount_of_known_children = station
                        stations_with_same_amount_of_known_children_as_less = []
                    else:
                        if station.amount_of_known_children_inside(child) == \
                            station_with_less_amount_of_known_children.amount_of_known_children_inside(child) and \
                            not child.been_on_station(station):
                            stations_with_same_amount_of_known_children_as_less.append(station)

                station_with_less_amount_of_children = station_with_less_amount_of_known_children
                if len(stations_with_same_amount_of_known_children_as_less) > 0:
                    for pot_station in stations_with_same_amount_of_known_children_as_less:
                        if pot_station.amount_of_children_inside() < station_with_less_amount_of_children.amount_of_children_inside():
                            station_with_less_amount_of_children = pot_station

                #На случай, если не вошли ни в один поиск
                if station_with_less_amount_of_children == self.__stations[0]:
                    if child.been_on_station(station_with_less_amount_of_children):
                        for station in self.__stations:
                            if not child.been_on_station(station):
                                station_with_less_amount_of_children = station

                station_with_less_amount_of_children.set_child(child)

            for station in self.__stations:
                station.clear()

camp = Camp()
camp.clock()
print("Маршруты:")
camp.print()
print("")
print("Загрузка станций:")
camp.checkup_stations_load()