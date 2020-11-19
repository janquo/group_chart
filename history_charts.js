var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([987])
             chartData.push([0])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160, 223, 195, 174, 188, 115, 123, 121, 384, 221, 208, 306, 222, 714, 619])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18, 9, 16, 10, 17, 41, 31, 27, 3, 7, 7, 3, 8, 0, 1])
chartData.push([430])
             chartData.push([2])
chartData.push([134, 216, 226, 313, 105, 220, 101, 134, 91, 193, 148, 213, 122, 93, 87, 0, 0, 0, 0, 0, 0, 221, 0, 0, 137, 158, 504, 180, 239, 132, 0, 0, 545, 388])
             chartData.push([20, 8, 6, 2, 35, 7, 30, 20, 40, 12, 24, 11, 32, 44, 47, 50, 50, 50, 50, 50, 50, 12, 50, 50, 28, 14, 2, 13, 4, 28, 50, 50, 1, 3])
chartData.push([96, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 373])
             chartData.push([49, 50, 20, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 4])
chartData.push([144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 349])
             chartData.push([22, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 5])
chartData.push([205])
             chartData.push([6])
chartData.push([201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 439])
             chartData.push([24, 50, 50, 50, 50, 50, 50, 50, 50, 50, 7])
chartData.push([323, 0, 0, 0, 0, 0, 0, 0, 0, 594])
             chartData.push([15, 50, 50, 50, 50, 50, 50, 50, 50, 8])
chartData.push([283])
             chartData.push([9])
chartData.push([442, 122, 0, 0, 0, 0, 0, 0, 0, 266])
             chartData.push([0, 32, 50, 50, 50, 50, 50, 50, 50, 10])
chartData.push([754, 331, 316, 350])
             chartData.push([1, 9, 10, 11])
chartData.push([128, 0, 155, 123, 151, 141, 154, 190, 0, 432, 106, 158, 180, 161, 142, 0, 93, 0, 134, 0, 0, 317, 169, 126, 248, 149, 107, 0, 116, 101, 0, 124, 191, 228, 218])
             chartData.push([20, 50, 16, 26, 11, 23, 20, 9, 50, 1, 28, 21, 14, 15, 19, 50, 49, 50, 38, 50, 50, 0, 24, 39, 6, 26, 41, 50, 47, 41, 50, 37, 12, 6, 12])
chartData.push([239])
             chartData.push([13])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75, 104, 91, 71, 178, 118, 85, 57, 82, 84, 56, 67, 65, 61, 107])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21, 11, 18, 20, 1, 4, 9, 32, 18, 18, 40, 33, 33, 46, 14])
chartData.push([125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 222, 0, 0, 237])
             chartData.push([26, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 15, 15, 50, 50, 15])
chartData.push([64, 0, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135])
             chartData.push([42, 50, 50, 50, 50, 50, 25, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 16])
chartData.push([122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 155, 0, 0, 0, 0, 0, 0, 0, 139, 0, 0, 0, 200])
             chartData.push([28, 50, 50, 50, 50, 50, 50, 50, 50, 50, 44, 33, 50, 50, 50, 50, 50, 50, 50, 50, 50, 41, 37, 50, 50, 50, 50, 50, 50, 50, 32, 50, 50, 50, 17])
chartData.push([1080, 277, 254])
             chartData.push([0, 9, 18])
chartData.push([208])
             chartData.push([19])
chartData.push([92, 0, 0, 0, 0, 0, 106, 75, 0, 0, 0, 0, 151, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 169, 0, 109, 155, 151])
             chartData.push([31, 50, 50, 50, 50, 50, 26, 44, 50, 50, 50, 50, 17, 50, 46, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 13, 50, 40, 19, 20])
chartData.push([139, 156, 0, 184])
             chartData.push([35, 25, 50, 21])
chartData.push([123, 117, 174, 0, 152, 190, 0, 0, 0, 124, 0, 0, 0, 181, 103, 0, 0, 0, 157, 0, 0, 118, 0, 157, 250, 211, 165, 0, 0, 117, 0, 0, 0, 228, 196])
             chartData.push([30, 36, 17, 50, 16, 15, 50, 50, 50, 31, 50, 50, 50, 18, 48, 50, 50, 50, 40, 50, 50, 47, 50, 31, 12, 13, 26, 50, 50, 44, 50, 50, 50, 12, 22])
chartData.push([124, 0, 0, 0, 0, 0, 0, 0, 0, 125, 0, 65, 0, 0, 53, 0, 0, 65, 207, 0, 121, 154, 278, 0, 0, 211, 385, 139, 0, 0, 67, 87, 82, 0, 96])
             chartData.push([9, 50, 50, 50, 50, 50, 50, 50, 50, 7, 50, 37, 50, 50, 45, 50, 50, 33, 3, 50, 10, 5, 3, 50, 50, 1, 0, 4, 50, 50, 41, 25, 28, 50, 23])
chartData.push([117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155])
             chartData.push([43, 50, 50, 50, 50, 50, 50, 50, 50, 50, 24])
chartData.push([189, 0, 419, 372, 236, 395, 296, 191, 170, 137, 0, 0, 0, 164])
             chartData.push([20, 50, 1, 2, 7, 1, 3, 16, 21, 34, 50, 50, 50, 25])
chartData.push([227, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 162])
             chartData.push([8, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26])
chartData.push([125, 144, 217, 0, 222, 148, 146, 0, 0, 0, 130, 169, 160, 0, 112, 0, 0, 0, 0, 0, 0, 157, 0, 0, 0, 0, 159, 0, 0, 151, 180, 259, 216, 188])
             chartData.push([37, 33, 14, 50, 12, 28, 25, 50, 50, 50, 43, 30, 31, 50, 48, 50, 50, 50, 50, 50, 50, 48, 50, 50, 50, 50, 26, 50, 50, 37, 29, 11, 22, 27])
chartData.push([114])
             chartData.push([28])
chartData.push([90, 172, 0, 0, 0, 0, 0, 127, 0, 324, 227, 0, 0, 193, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156])
             chartData.push([49, 16, 50, 50, 50, 50, 50, 33, 50, 5, 11, 50, 50, 20, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 29])
chartData.push([177, 164, 192, 303, 0, 0, 150, 155, 154])
             chartData.push([10, 11, 15, 2, 50, 50, 29, 36, 30])
chartData.push([86, 96, 0, 0, 0, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 162, 100, 118, 121])
             chartData.push([40, 38, 50, 50, 50, 16, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 17, 48, 41, 31])
chartData.push([81, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 85, 0, 0, 0, 109, 239, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 106])
             chartData.push([43, 50, 50, 50, 50, 50, 50, 50, 39, 50, 50, 50, 32, 50, 50, 50, 36, 5, 14, 50, 50, 50, 50, 50, 50, 50, 50, 50, 45, 50, 50, 50, 32])
chartData.push([112, 192, 151, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118])
             chartData.push([26, 6, 14, 50, 50, 31, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 33])
chartData.push([99, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 205, 0, 0, 0, 127])
             chartData.push([35, 50, 34, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 50, 50, 50, 34])
chartData.push([172, 135, 253, 0, 0, 262, 109, 135, 143, 0, 0, 0, 175, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 216, 0, 0, 0, 0, 0, 0, 167, 0, 164])
             chartData.push([22, 40, 9, 50, 50, 11, 43, 42, 28, 50, 50, 50, 24, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 48, 17, 50, 50, 50, 50, 50, 50, 34, 50, 35])
chartData.push([118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 109, 0, 0, 106, 0, 0, 180, 126])
             chartData.push([25, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 43, 36, 50, 50, 46, 50, 50, 17, 36])
chartData.push([182, 135])
             chartData.push([23, 37])
chartData.push([247, 175, 185, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 229, 0, 0, 289, 0, 0, 0, 0, 0, 208])
             chartData.push([13, 31, 36, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 31, 50, 50, 17, 50, 50, 50, 50, 50, 38])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341, 457, 235, 294, 486, 182, 205, 182, 300, 207, 225, 0, 0, 312, 207])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12, 3, 40, 11, 5, 48, 34, 38, 14, 31, 27, 50, 50, 16, 39])
chartData.push([48])
             chartData.push([40])
chartData.push([791, 0, 0, 0, 263, 188])
             chartData.push([1, 50, 50, 50, 24, 41])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259, 217, 277, 190, 0, 252, 0, 0, 171, 0, 128, 0, 240, 176, 0, 140])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10, 15, 8, 28, 50, 11, 50, 50, 16, 50, 37, 50, 13, 24, 50, 42])
chartData.push([346, 0, 150])
             chartData.push([6, 50, 43])
chartData.push([173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149])
             chartData.push([21, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 44])
chartData.push([252, 0, 0, 69, 0, 0, 0, 59, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89, 72, 0, 0, 0, 0, 0, 0, 0, 80])
             chartData.push([2, 50, 50, 35, 50, 50, 50, 42, 50, 50, 50, 31, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 36, 46, 50, 50, 50, 50, 50, 50, 50, 45])
chartData.push([82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 0, 0, 104, 0, 0, 125, 249, 113])
             chartData.push([48, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 42, 50, 50, 50, 50, 39, 50, 50, 37, 5, 46])
chartData.push([110])
             chartData.push([47])
chartData.push([316, 103, 0, 172, 0, 0, 0, 0, 0, 95, 0, 0, 100, 0, 0, 0, 153, 0, 0, 168, 175, 125, 188, 162, 0, 0, 0, 0, 0, 0, 125, 0, 120])
             chartData.push([2, 31, 50, 10, 50, 50, 50, 50, 50, 48, 50, 50, 39, 50, 50, 50, 26, 50, 50, 21, 21, 40, 18, 20, 50, 50, 50, 50, 50, 50, 46, 50, 48])


for (i = 0; i < charts_number; i++) {
    chartHandles.push(document.getElementById("chart" + i));

    chartSettings.push({
        labels: new Array(chartData[i * 2].length).fill(":)"),
        datasets: [{
            data: chartData[i * 2],
            label: "Scrobbles",
            yAxisID: 'A',
            borderColor: colors[1],
            borderWidth: 4,
            pointBackgroundColor: colors[1],
            backgroundColor: colors[3],
        },
        {
            data: chartData[i * 2 + 1],
            backgroundColor: 'transparent',
            label: "position",
            yAxisID: 'B',
            borderColor: colors[0],
            borderWidth: 4,
            pointBackgroundColor: colors[0]
        }]
    });

    if (chartHandles[i]) {
        new Chart(chartHandles[i], {
            type: 'line',
            data: chartSettings[i],
            options: {
                scales: {
                    yAxes: [{
                        id: 'A',
                        type: 'linear',
                        position: 'left',
                        ticks: {
                            precision: 0,
                        }
                    }, {
                        id: 'B',
                        type: 'linear',
                        position: 'right',
                        ticks: {
                            max: charts_number,
                            min: 1,
                            reverse: true
                        }
                    }]
                },
                legend: {
                    display: false
                }
            }
        });
    }
}