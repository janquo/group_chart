var colors = ['#007bff', '#28a745', '#333333', '#c3e6cb', '#dc3545', '#6c757d'];

var charts_number = 49;

var chartHandles = [];
var chartSettings = [];
var chartData = [];

chartData.push([2212])
             chartData.push([0])
chartData.push([1447])
             chartData.push([1])
chartData.push([319, 193, 124, 188, 138, 115, 129, 96, 0, 247, 0, 0, 0, 169, 112, 140, 167, 265, 171, 187, 496])
             chartData.push([3, 9, 27, 10, 12, 28, 22, 34, 50, 4, 50, 50, 50, 12, 29, 20, 15, 4, 19, 16, 2])
chartData.push([100, 0, 98, 133, 0, 136, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 184, 126, 0, 332])
             chartData.push([27, 50, 35, 19, 50, 19, 50, 50, 50, 50, 46, 50, 50, 50, 50, 50, 50, 7, 33, 50, 3])
chartData.push([325, 298])
             chartData.push([3, 4])
chartData.push([205])
             chartData.push([5])
chartData.push([52, 51, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 159, 77, 60, 229, 0, 0, 0, 0, 152])
             chartData.push([43, 46, 50, 50, 50, 50, 50, 36, 50, 50, 50, 50, 7, 26, 38, 2, 50, 50, 50, 50, 6])
chartData.push([315, 276])
             chartData.push([7, 7])
chartData.push([227])
             chartData.push([8])
chartData.push([149, 0, 0, 0, 0, 0, 0, 190, 201, 370, 257, 0, 0, 232, 398, 0, 197, 290, 309])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 22, 21, 6, 16, 50, 50, 14, 4, 50, 39, 13, 9])
chartData.push([124, 0, 0, 0, 0, 0, 0, 0, 0, 125, 0, 65, 0, 0, 53, 0, 0, 65, 207, 0, 121])
             chartData.push([9, 50, 50, 50, 50, 50, 50, 50, 50, 7, 50, 37, 50, 50, 45, 50, 50, 33, 3, 50, 10])
chartData.push([344, 194, 165, 159, 206, 205, 138, 152, 0, 0, 0, 171, 114, 192, 186, 0, 151, 136, 246, 148, 241])
             chartData.push([4, 14, 20, 22, 7, 11, 27, 22, 50, 50, 50, 25, 49, 16, 17, 50, 25, 28, 14, 39, 11])
chartData.push([336, 426, 322, 286, 301, 386, 479, 339, 294, 228, 0, 229, 275, 460, 467, 404, 253, 285, 239, 305, 341])
             chartData.push([10, 4, 11, 12, 6, 6, 3, 6, 11, 20, 50, 27, 20, 3, 8, 5, 17, 11, 31, 17, 12])
chartData.push([40])
             chartData.push([13])
chartData.push([81, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 85, 0, 0, 0, 109, 239, 153])
             chartData.push([43, 50, 50, 50, 50, 50, 50, 50, 39, 50, 50, 50, 32, 50, 50, 50, 36, 5, 14])
chartData.push([103, 0, 136, 293, 0, 0, 0, 264, 158, 0, 0, 274, 253, 308, 0, 192, 126, 0, 0, 259, 217])
             chartData.push([45, 50, 30, 3, 50, 50, 50, 4, 25, 50, 50, 8, 12, 5, 50, 11, 39, 50, 50, 10, 15])
chartData.push([86, 96, 0, 0, 0, 153])
             chartData.push([40, 38, 50, 50, 50, 16])
chartData.push([277, 1128, 0, 0, 0, 0, 0, 0, 0, 0, 237, 256, 0, 181, 270, 0, 281])
             chartData.push([9, 1, 50, 50, 50, 50, 50, 50, 50, 50, 21, 18, 50, 36, 23, 50, 17])
chartData.push([99, 100, 126, 0, 0, 331, 208, 194, 133, 128, 110, 97, 0, 126, 95, 0, 0, 110, 238, 283, 160])
             chartData.push([34, 34, 25, 50, 50, 2, 10, 7, 22, 21, 27, 46, 50, 29, 43, 50, 50, 31, 8, 6, 18])
chartData.push([140, 0, 0, 0, 174])
             chartData.push([24, 50, 50, 50, 19])
chartData.push([205])
             chartData.push([20])
chartData.push([126, 91, 87, 59, 57, 73, 96, 81, 78, 52, 53, 60, 107, 51, 66, 54, 0, 59, 0, 65, 75])
             chartData.push([5, 10, 12, 27, 22, 20, 14, 12, 15, 30, 29, 32, 10, 40, 23, 30, 50, 25, 50, 37, 21])
chartData.push([322, 0, 0, 0, 194])
             chartData.push([5, 50, 50, 50, 22])
chartData.push([412, 162])
             chartData.push([2, 23])
chartData.push([113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 280, 0, 0, 184, 176])
             chartData.push([37, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 7, 50, 50, 24, 24])
chartData.push([196, 955, 556, 170, 143, 168, 0, 144, 161])
             chartData.push([15, 1, 2, 15, 22, 14, 50, 35, 25])
chartData.push([166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 190])
             chartData.push([20, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 26])
chartData.push([1150, 267, 161, 155, 0, 254, 224])
             chartData.push([1, 10, 41, 49, 50, 21, 27])
chartData.push([67, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 70, 81])
             chartData.push([30, 50, 50, 50, 32, 50, 50, 50, 50, 50, 50, 50, 34, 50, 50, 50, 50, 50, 42, 28])
chartData.push([114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161])
             chartData.push([37, 50, 50, 50, 50, 50, 50, 50, 50, 50, 29])
chartData.push([208, 229, 222, 0, 0, 0, 0, 500, 387, 335, 384, 145, 137, 141, 0, 0, 0, 0, 203, 162, 147])
             chartData.push([11, 8, 10, 50, 50, 50, 50, 2, 4, 3, 4, 29, 34, 27, 50, 50, 50, 50, 16, 28, 30])
chartData.push([154])
             chartData.push([31])
chartData.push([168, 141])
             chartData.push([26, 32])
chartData.push([179, 0, 147])
             chartData.push([28, 50, 33])
chartData.push([257])
             chartData.push([34])
chartData.push([395, 201, 205])
             chartData.push([9, 41, 35])
chartData.push([182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 323, 264])
             chartData.push([47, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 30, 36])
chartData.push([109, 0, 0, 0, 0, 0, 123, 0, 96, 0, 0, 225, 0, 132])
             chartData.push([32, 50, 50, 50, 50, 50, 36, 50, 46, 50, 50, 15, 50, 37])
chartData.push([139, 105, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119])
             chartData.push([19, 38, 36, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 38])
chartData.push([127])
             chartData.push([39])
chartData.push([53, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 56])
             chartData.push([30, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 46, 50, 50, 50, 40])
chartData.push([119])
             chartData.push([41])
chartData.push([186, 240, 118])
             chartData.push([20, 9, 42])
chartData.push([85])
             chartData.push([43])
chartData.push([324, 0, 0, 167, 273, 137, 0, 0, 0, 0, 0, 0, 0, 0, 140, 140, 0, 0, 158, 148])
             chartData.push([6, 50, 50, 18, 8, 40, 50, 50, 50, 50, 50, 50, 50, 50, 36, 42, 50, 50, 45, 44])
chartData.push([74])
             chartData.push([45])
chartData.push([115])
             chartData.push([46])
chartData.push([115, 0, 182, 206, 167, 271, 330, 353, 134, 114])
             chartData.push([40, 50, 13, 13, 17, 6, 3, 4, 40, 47])
chartData.push([174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103])
             chartData.push([17, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 48])


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