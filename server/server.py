from flask import Flask, request, jsonify
import easyocr
import numpy
import json

def convert(o):
    if isinstance(o, numpy.int32): return int(o)  
    raise TypeError

app = Flask(__name__)
reader = easyocr.Reader(['ch_sim'])

@app.route('/ocr', methods=['POST'])
def perform_ocr():
    image_data = request.data
    result = reader.readtext(image_data)
    print(result)
    # [([[42, 0], [184, 0], [184, 48], [42, 48]], '1 |汊字', 0.7368230691578994)]
    boxes = []
    texts = []
    thisbox = []
    for item in result:
        thisbox = []
        for cord in item[0]:
            thisbox.append((int(cord[0]), int(cord[1])))
        boxes.append(thisbox)
        texts.append(item[1])

    return json.dumps({'boxes': boxes, 'texts': texts})
    


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=7272)
