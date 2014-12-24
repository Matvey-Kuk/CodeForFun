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

    //Массив свойств клетки 2-движущийся объект 1-стена 0-путь свободен
    var x_y_block_types_array=new Array(quontity_of_blocks_x);
    for (i=0; i<quontity_of_blocks_x; i++) {
        x_y_block_types_array[i] = new Array(quontity_of_blocks_y);
    }

    for(i = 0 ; i < x_y_block_types_array.length ; i++){
        for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
            x_y_block_types_array[i][j] = {
                type: 'field',
                distance: ''
            };
        }
    }

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

                if(x_y_block_types_array[i][j]['type'] == 'way'){
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

    //Определяем на какую клетку нажал пользователь по координатам мыши
    var mouseClickListener = function(event) {

        for(i = 0 ; i < x_y_block_types_array.length ; i++){
                for(j = 0 ; j < x_y_block_types_array[0].length ; j++) {
                        x_y_block_types_array[i][j]['distance'] = '';
                }
            }

        var posX = event.clientX - canvas.offsetLeft;
        var posY = event.clientY - canvas.offsetTop;
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
        count_distances();
    });

    $( "#set_mode_start" ).click(function() {
        edit_mode = 'start';
    });

    $( "#set_mode_end" ).click(function() {
        edit_mode = 'end';
    });

    $( "#set_mode_border" ).click(function() {
        edit_mode = 'border';
    });

    var count_distances = function(){
        var shift = [-1, 0, 1];
        var a, b;
        var counted_one_item = false;
        var some_filed_distance_changed = true;
        for(i = 0 ; i < x_y_block_types_array.length ; i++){
            for(j = 0 ; j < x_y_block_types_array[0].length && !counted_one_item ; j++) {
                if(x_y_block_types_array[i][j]['type'] == 'field'){
                    var min_distance = x_y_block_types_array[i][j]['distance'];
                    for(a = 0 ; a < shift.length ; a++){
                        for(b = 0 ; b < shift.length; b++){
                            if((i + shift[a] >= 0) &&  (j + shift[b] >= 0) && (i + shift[a] < quontity_of_blocks_x) && (j + shift[b] < quontity_of_blocks_y) && !((shift[a] == 0) && (shift[b] == 0))){
                                if(min_distance == ""){
                                    if(x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] != ''){
                                        min_distance = x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + 1;
                                    }
                                } else {
                                    if(x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] != "") {
                                        if (x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + 1 < min_distance) {
                                            var apply = 1;
                                            if(shift[a] * shift[b] != 0){
                                                apply = 1.4;
                                            }
                                            min_distance = x_y_block_types_array[i + shift[a]][j + shift[b]]['distance'] + apply;
                                        }
                                    }
                                }
                                if(x_y_block_types_array[i + shift[a]][j + shift[b]]['type'] == 'start'){
                                    var apply = 1;
                                    if(shift[a] * shift[b] != 0){
                                        apply = 1.4;
                                    }
                                    min_distance = apply;
                                }
                            }
                        }
                    }
                    if((x_y_block_types_array[i][j]['distance'] == '') || min_distance < x_y_block_types_array[i][j]['distance']){
                        x_y_block_types_array[i][j]['distance'] = min_distance;
                        if(min_distance != ''){
                            counted_one_item = true;
                        }
                    }
                }
            }
        }
        make_grid();
    };

    //Слушаю все нажатия мыши на холсте и вызываю функцию обработчик
    canvas.addEventListener("mousedown", mouseClickListener, false);
});