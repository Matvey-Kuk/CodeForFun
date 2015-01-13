//Ждем конца загрузки страницы
$(document).ready(function(){
    var i, j;

    var edit_mode = 'border';

    var canvas = document.getElementById("c");
    var ctx = canvas.getContext("2d");

    //Блок, в котором лежит канвас
    var canvasDiv = $('#canvas-div');

//    делаем размер канваса такой же как размер родительского блока
    var resize_canvas = function(){
        ctx.canvas.width  = canvasDiv.width();
        ctx.canvas.height = canvasDiv.height();
    };

    resize_canvas();

    //Количество блоков по X, Y
    var quontity_of_blocks_x = 20;
    var quontity_of_blocks_y = 20;

    var x_y_block_types_array = NaN;

    var make_internal_data = function(){
        //Массив свойств клетки 2-движущийся объект 1-стена 0-путь свободен
        x_y_block_types_array=new Array(quontity_of_blocks_x);
        for (i=0; i<quontity_of_blocks_x; i++) {
            x_y_block_types_array[i] = new Array(parseInt(quontity_of_blocks_y, 10));
        }

        for(i = 0 ; i < x_y_block_types_array.length ; i++){
            for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                x_y_block_types_array[i][j] = {
                    type: 'field',
                    distance: ''
                };
            }
        }
    };

    make_internal_data();

    //Рисует сетку
    var make_grid = function(){
        var rectangle_x_size = canvasDiv.width() / quontity_of_blocks_x;
        var rectangle_y_size = canvasDiv.height() / quontity_of_blocks_y;
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for(var i = 0 ; i < quontity_of_blocks_x ; i++) {
            for (var j = 0; j < quontity_of_blocks_y; j++) {
                var color;

                if(x_y_block_types_array[i][j]['type'] == 'field'){
                    color = "grey";
                }

                if(x_y_block_types_array[i][j]['type'] == 'border'){
                    color = "black";
                }

                if(x_y_block_types_array[i][j]['type'] == 'start'){
                    color = 'green';
                }

                if(x_y_block_types_array[i][j]['type'] == 'end'){
                    color = 'purple';
                }

                if(x_y_block_types_array[i][j]['type'] == 'route'){
                    color = 'blue';
                }

                ctx.fillStyle = color;
                ctx.fillRect(
                        i * rectangle_x_size,
                        j * rectangle_y_size,
                        rectangle_x_size - 2,
                        rectangle_y_size - 2
                );

                ctx.fillStyle = 'white';
                ctx.font="10px Arial";
                ctx.fillText(x_y_block_types_array[i][j]['distance'],i * rectangle_x_size + 10,j * rectangle_y_size + 20);
            }
        }
    };

    make_grid();

    var clear_work_results = function(){
        for(i = 0 ; i < x_y_block_types_array.length ; i++){
            for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                x_y_block_types_array[i][j]['distance'] = '';
            }
        }

        for(i = 0 ; i < x_y_block_types_array.length ; i++){
            for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                if(x_y_block_types_array[i][j]['type'] == 'route'){
                    x_y_block_types_array[i][j]['type'] = 'field';
                }
            }
        }
    };

    //Определяем на какую клетку нажал пользователь по координатам мыши
    var mouseClickListener = function(event) {
        clear_work_results();

        var posX = event.clientX - canvas.offsetLeft;
        var posY = event.clientY - canvas.offsetTop + $(window).scrollTop();
        var rectangle_x_size = canvasDiv.width() / quontity_of_blocks_x;
        var rectangle_y_size = canvasDiv.height() / quontity_of_blocks_y;
        var target_coords = {
            x: Math.floor(posX / rectangle_x_size),
            y: Math.floor(posY / rectangle_y_size)
        };

        if(edit_mode == 'border') {
            if (x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] == 'field') {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'border';
            } else {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'field';
            }
        }

        if(edit_mode == 'start') {
            for(i = 0 ; i < x_y_block_types_array.length ; i++){
                for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                    if(x_y_block_types_array[i][j]['type'] == 'start'){
                        x_y_block_types_array[i][j]['type'] = 'field';
                    }
                }
            }
            if (x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] == 'field') {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'start';
            } else {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'field';
            }
        }

        if(edit_mode == 'end') {
            for(i = 0 ; i < x_y_block_types_array.length ; i++){
                for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                    if(x_y_block_types_array[i][j]['type'] == 'end'){
                        x_y_block_types_array[i][j]['type'] = 'field';
                    }
                }
            }
            if (x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] == 'field') {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'end';
            } else {
                x_y_block_types_array[target_coords['x']][target_coords['y']]['type'] = 'field';
            }
        }


        make_grid();
    };

    $( "#show_process_button" ).click(function() {
        clear_work_results();
        count_distances();
        make_route(true);
    });

    $( "#show_result_button" ).click(function() {
        clear_work_results();
        count_distances();
        make_route(false);
    });

    $( "#set_mode_start" ).click(function() {
        clear_work_results();
        edit_mode = 'start';
    });

    $( "#set_mode_end" ).click(function() {
        clear_work_results();
        edit_mode = 'end';
    });

    $( "#set_mode_border" ).click(function() {
        clear_work_results();
        edit_mode = 'border';
    });

    var dump_area = $("#dump_area");

    $( "#import_map" ).click(function() {
        clear_work_results();
        var lines = dump_area.val().split("\n");
        for(var i = 0 ; i < lines.length ; i++){
            var line_elements = lines[i].split("\t");
            line_elements[0] = parseInt(line_elements[0], 10);
            line_elements[1] = parseInt(line_elements[1], 10);
            if(line_elements.length == 3){
                x_y_block_types_array[line_elements[0]][line_elements[1]]['type'] = line_elements[2];
            }
        }
        make_grid()
    });

    $( "#export_map" ).click(function() {
        clear_work_results();
        var dump_string = "";
        for(i = 0 ; i < x_y_block_types_array.length ; i++){
            for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                dump_string += i + "\t" + j + "\t" + x_y_block_types_array[i][j]['type'] + "\n";
            }
        }
        dump_area.html(dump_string);
    });

    var inputXSize = $("#inputXSize");
    var inputYSize = $("#inputYSize");

    $( "#resizeButton" ).click(function() {
        clear_work_results();
        quontity_of_blocks_x = parseFloat(inputXSize.val());
        quontity_of_blocks_y = parseFloat(inputYSize.val());

        resize_canvas();
        make_internal_data();
        make_grid();
    });

    var count_distances = function(){
        var shift = [-1, 0, 1];
        var a, b;
        var counted_one_item = true;
        while (counted_one_item) {
            counted_one_item = false;
            for (i = 0; i < x_y_block_types_array.length; i++) {
                for (j = 0; j < x_y_block_types_array[0].length; j++) {
                    if (x_y_block_types_array[i][j]['type'] == 'field') {
                        var min_distance = x_y_block_types_array[i][j]['distance'];
                        for (a = 0; a < shift.length; a++) {
                            for (b = 0; b < shift.length; b++) {
                                if ((i + shift[a] >= 0) && (j + shift[b] >= 0) && (i + shift[a] < quontity_of_blocks_x) && (j + shift[b] < quontity_of_blocks_y) && !((shift[a] == 0) && (shift[b] == 0)) && (shift[a] * shift[b] == 0)) {
                                    if (min_distance == "") {
                                        if (x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] != '') {
                                            min_distance = x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + 1;
                                        }
                                    } else {
                                        if (x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] != "") {
                                            if (x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + 1 < min_distance) {
                                                var apply = 1;
                                                if (shift[a] * shift[b] != 0) {
                                                    apply = 1.4;
                                                }
                                                min_distance = x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + apply;
                                            }
                                        }
                                    }
                                    if (x_y_block_types_array[i + shift[a]][j + shift[b]]['type'] == 'start') {
                                        var apply = 1;
                                        if (shift[a] * shift[b] != 0) {
                                            apply = 1.4;
                                        }
                                        min_distance = apply;
                                    }
                                }
                            }
                        }
                        if ((x_y_block_types_array[i][j]['distance'] == '') || min_distance < x_y_block_types_array[i][j]['distance']) {
                            x_y_block_types_array[i][j]['distance'] = min_distance;
                            if (min_distance != '') {
                                counted_one_item = true;
                            }
                        }
                    }
                }
            }
        }
    };

    var searchTimeResults = $("#searchTimeResults");

    var make_route = function(animated){

        var start_time = Date.now();

        var shift = [-1, 0, 1];

        var end_point = {x: NaN, y: NaN};
        for (i = 0; i < x_y_block_types_array.length; i++) {
            for (j = 0; j < x_y_block_types_array[0].length; j++) {
                if(x_y_block_types_array[i][j]['type'] == 'end'){
                    end_point['x'] = i;
                    end_point['y'] = j;
                }
            }
        }

        var finish_has_been_founded = false;
        var constraint = 50;
        var i = end_point['x'];
        var j = end_point['y'];
        var next_route_point_has_been_found = true;

        var find_next_route_point = function () {
            if (!finish_has_been_founded && next_route_point_has_been_found) {
                next_route_point_has_been_found = false;
                var minimal_distance = NaN;
                var min_dist_i = NaN;
                var min_dist_j = NaN;

                for (var a = 0; a < shift.length; a++) {
                    for (var b = 0; b < shift.length; b++) {
                        if ((i + shift[a] >= 0) && (j + shift[b] >= 0) && (i + shift[a] < quontity_of_blocks_x) && (j + shift[b] < quontity_of_blocks_y) && !((shift[a] == 0) && (shift[b] == 0)) && (shift[a] * shift[b] == 0)) {
                            if (x_y_block_types_array[i + shift[a]][j + shift[b]]['type'] == 'field' && (x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] != '')) {
                                var current_point_distance = x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'];
                                if(isNaN(minimal_distance) || minimal_distance > current_point_distance){
                                    minimal_distance = current_point_distance;
                                    min_dist_i = i + shift[a];
                                    min_dist_j = j + shift[b];
                                }
                            }
                            if(x_y_block_types_array[i + shift[a]][j + shift[b]]['type'] == 'start'){
                                finish_has_been_founded = true;
                            }
                        }
                    }
                }

                if(!finish_has_been_founded && !isNaN(minimal_distance)){
                    x_y_block_types_array[min_dist_i][min_dist_j]['type'] = 'route';
                    i = min_dist_i;
                    j = min_dist_j;
                    next_route_point_has_been_found = true;
                }

                if(isNaN(minimal_distance)){
                    alert('Невозможно найти путь.');
                }

                make_grid();

                if(animated){
                    setTimeout(find_next_route_point, 100);
                } else {
                    find_next_route_point();
                }
            }
        };

        find_next_route_point();
        searchTimeResults.html(Date.now() - start_time);
    };

    //Слушаю все нажатия мыши на холсте и вызываю функцию обработчик
    canvas.addEventListener("mousedown", mouseClickListener, false);
});