var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([684, 883, 399])
             chartData.push([0, 0, 0])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75, 104, 91, 71, 178])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21, 11, 18, 20, 1])
chartData.push([189, 0, 419, 372])
             chartData.push([20, 50, 1, 2])
chartData.push([48, 0, 0, 0, 0, 0, 104, 95, 0, 0, 0, 0, 99, 188, 188])
             chartData.push([48, 50, 50, 50, 50, 50, 13, 13, 50, 50, 50, 50, 25, 2, 3])
chartData.push([194, 0, 0, 0, 278, 230, 145, 80, 0, 0, 0, 0, 0, 0, 215])
             chartData.push([6, 50, 50, 50, 4, 4, 9, 26, 50, 50, 50, 50, 50, 50, 4])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341, 457, 235, 294, 486])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12, 3, 40, 11, 5])
chartData.push([128, 0, 155, 123, 151, 141, 154, 190, 0, 432, 106, 158, 180, 161, 142, 0, 93, 0, 134, 0, 0, 317, 169, 126, 248])
             chartData.push([20, 50, 16, 26, 11, 23, 20, 9, 50, 1, 28, 21, 14, 15, 19, 50, 49, 50, 38, 50, 50, 0, 24, 39, 6])
chartData.push([319, 193, 124, 188, 138, 115, 129, 96, 0, 247, 0, 0, 0, 169, 112, 140, 167, 265, 171, 187, 496, 231, 254, 158, 242])
             chartData.push([3, 9, 27, 10, 12, 28, 22, 34, 50, 4, 50, 50, 50, 12, 29, 20, 15, 4, 19, 16, 2, 7, 8, 13, 7])
chartData.push([163, 227])
             chartData.push([12, 8])
chartData.push([195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 0, 220, 0, 0, 0, 0, 0, 218])
             chartData.push([5, 50, 50, 50, 50, 50, 50, 50, 50, 50, 18, 50, 50, 50, 10, 50, 50, 50, 50, 50, 9])
chartData.push([325, 298, 141, 280, 176, 216])
             chartData.push([3, 4, 26, 6, 9, 10])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259, 217, 277, 190, 0, 252])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10, 15, 8, 28, 50, 11])
chartData.push([123, 117, 174, 0, 152, 190, 0, 0, 0, 124, 0, 0, 0, 181, 103, 0, 0, 0, 157, 0, 0, 118, 0, 157, 250])
             chartData.push([30, 36, 17, 50, 16, 15, 50, 50, 50, 31, 50, 50, 50, 18, 48, 50, 50, 50, 40, 50, 50, 47, 50, 31, 12])
chartData.push([200])
             chartData.push([13])
chartData.push([135])
             chartData.push([14])
chartData.push([114])
             chartData.push([15])
chartData.push([208])
             chartData.push([16])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160, 223, 195, 174, 188])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18, 9, 16, 10, 17])
chartData.push([316, 103, 0, 172, 0, 0, 0, 0, 0, 95, 0, 0, 100, 0, 0, 0, 153, 0, 0, 168, 175, 125, 188])
             chartData.push([2, 31, 50, 10, 50, 50, 50, 50, 50, 48, 50, 50, 39, 50, 50, 50, 26, 50, 50, 21, 21, 40, 18])
chartData.push([169])
             chartData.push([19])
chartData.push([1447, 308, 224, 209, 192])
             chartData.push([1, 1, 14, 7, 20])
chartData.push([344, 194, 165, 159, 206, 205, 138, 152, 0, 0, 0, 171, 114, 192, 186, 0, 151, 136, 246, 148, 241, 139, 173, 0, 209])
             chartData.push([4, 14, 20, 22, 7, 11, 27, 22, 50, 50, 50, 25, 49, 16, 17, 50, 25, 28, 14, 39, 11, 34, 33, 50, 21])
chartData.push([224])
             chartData.push([22])
chartData.push([173])
             chartData.push([23])
chartData.push([201])
             chartData.push([24])
chartData.push([1150, 267, 161, 155, 0, 254, 224, 265, 278, 224, 228])
             chartData.push([1, 10, 41, 49, 50, 21, 27, 22, 22, 23, 25])
chartData.push([324, 0, 0, 167, 273, 137, 0, 0, 0, 0, 0, 0, 0, 0, 140, 140, 0, 0, 158, 148, 0, 0, 0, 198])
             chartData.push([6, 50, 50, 18, 8, 40, 50, 50, 50, 50, 50, 50, 50, 50, 36, 42, 50, 50, 45, 44, 50, 50, 50, 26])
chartData.push([62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 269, 0, 0, 0, 0, 84])
             chartData.push([41, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 0, 50, 50, 50, 50, 27])
chartData.push([59, 0, 0, 70, 0, 0, 70, 0, 74, 105, 128, 89, 71, 0, 74, 101, 77, 0, 72, 0, 0, 97])
             chartData.push([48, 50, 50, 38, 50, 50, 32, 50, 39, 23, 8, 25, 33, 50, 37, 29, 48, 50, 42, 50, 50, 28])
chartData.push([318])
             chartData.push([29])
chartData.push([53, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 56, 73, 92, 60, 69])
             chartData.push([30, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 40, 24, 17, 46, 30])
chartData.push([298, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 149])
             chartData.push([3, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 17, 31])
chartData.push([94])
             chartData.push([32])
chartData.push([109, 0, 0, 0, 0, 0, 123, 0, 96, 0, 0, 225, 0, 132, 131, 0, 0, 143])
             chartData.push([32, 50, 50, 50, 50, 50, 36, 50, 46, 50, 50, 15, 50, 37, 33, 50, 50, 33])
chartData.push([157, 213, 280, 146, 120, 181, 214, 192, 132, 209, 0, 0, 130, 139, 106, 85, 0, 107, 129, 0, 0, 157, 151, 126, 129])
             chartData.push([14, 7, 4, 20, 17, 9, 8, 8, 23, 6, 50, 50, 32, 23, 33, 49, 50, 35, 41, 50, 50, 23, 31, 38, 34])
chartData.push([2212, 376, 378, 200, 206])
             chartData.push([0, 6, 10, 41, 35])
chartData.push([152])
             chartData.push([36])
chartData.push([260, 473, 155, 125])
             chartData.push([4, 2, 15, 37])
chartData.push([315, 276, 244, 180, 154, 149])
             chartData.push([7, 7, 13, 32, 33, 38])
chartData.push([203, 120, 0, 0, 0, 215, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172])
             chartData.push([16, 44, 50, 50, 50, 17, 46, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 39])
chartData.push([78, 96, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 48])
             chartData.push([10, 7, 3, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 42, 40])
chartData.push([121, 0, 112, 108, 124, 93, 158, 0, 0, 0, 0, 0, 0, 0, 0, 143, 0, 111, 171, 0, 132])
             chartData.push([25, 50, 35, 33, 33, 46, 19, 50, 50, 50, 50, 50, 50, 50, 50, 36, 50, 45, 29, 50, 41])
chartData.push([82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119])
             chartData.push([48, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 42])
chartData.push([117])
             chartData.push([43])
chartData.push([149, 0, 0, 0, 0, 0, 0, 190, 201, 370, 257, 0, 0, 232, 398, 0, 197, 290, 309, 262, 258, 0, 175])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 22, 21, 6, 16, 50, 50, 14, 4, 50, 39, 13, 9, 18, 23, 50, 44])
chartData.push([174])
             chartData.push([45])
chartData.push([138])
             chartData.push([46])
chartData.push([115, 0, 182, 206, 167, 271, 330, 353, 134, 114, 0, 0, 0, 124])
             chartData.push([40, 50, 13, 13, 17, 6, 3, 4, 40, 47, 50, 50, 50, 47])
chartData.push([172, 135, 253, 0, 0, 262, 109, 135, 143, 0, 0, 0, 175, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145])
             chartData.push([22, 40, 9, 50, 50, 11, 43, 42, 28, 50, 50, 50, 24, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 48])


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